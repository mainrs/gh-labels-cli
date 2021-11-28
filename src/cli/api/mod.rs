use crate::{
    cli::Cli,
    util::{create_github_api_client, get_github_repo_and_owner},
    Result,
};
use clap::Parser;
use eyre::{eyre, WrapErr};
use git2::Repository;
use std::env::current_dir;

mod create;
mod export;
mod update;

/// Interact with the GitHub API.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct ApiArgs {
    #[clap(subcommand)]
    pub cmd: ApiSubCommand,

    /// The number of concurrent connections to make to the GitHub API.
    #[clap(default_value("4"), long)]
    pub concurrent_connections: usize,

    /// The git repository to apply the changes to.
    ///
    /// Can be either a git url or a string in the format `owner/repo`. If not
    /// set, the current directory is assumed to be a valid git repository and
    /// the remote url named `origin` will be taken.
    #[clap(long, short)]
    pub repo: Option<String>,

    /// The GitHub personal access token. Takes precedence over environment
    /// variables.
    #[clap(long, short)]
    pub token: Option<String>,
}

#[derive(Clone, Debug, Parser)]
pub enum ApiSubCommand {
    Create(create::CreateArgs),
    Export(export::ExportArgs),
    Update(update::UpdateArgs),
}

impl ApiArgs {
    fn repo_details_from_cli_or_current_dir(&self) -> Result<String> {
        match &self.repo {
            Some(repo) => Ok(repo.to_string()),
            None => {
                // Try to see if the current working directory is a valid git
                // repository.
                let current_dir = current_dir()
                    .wrap_err_with(|| "Failed to determine current working directory")?;
                let repo = Repository::open(&current_dir)
                    .wrap_err_with(|| "Is the current working directory a valid git repository?")?;
                let remote = repo
                    .find_remote("origin")
                    .wrap_err_with(|| "The git repository does not have a remote named origin")?;
                remote
                    .url()
                    .ok_or_else(|| eyre!("The URL is not valid UTF-8"))
                    .map(Into::into)
            }
        }
    }

    pub async fn run(self, cli: Cli) -> Result<()> {
        let github = create_github_api_client(self.token.as_deref())?;
        let repo_raw = self.repo_details_from_cli_or_current_dir()?;
        let repo = get_github_repo_and_owner(&repo_raw)?;
        let repo = github.repo(repo.0, repo.1);

        match self.cmd {
            ApiSubCommand::Create(args) => args.run(cli, repo, repo_raw).await?,
            ApiSubCommand::Export(args) => args.run(cli, repo).await?,
            ApiSubCommand::Update(args) => args.run(cli, repo, self.concurrent_connections).await?,
        }

        Ok(())
    }
}
