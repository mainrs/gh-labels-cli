use crate::{
    cli::Cli,
    error::Error,
    extension::{LabelAlreadyExistsExt, RepoNotFoundExt, UnauthorizedExt},
    file::{read_from_cli_arg_or_fallback_to_config_dir, JsonLabel},
    Result,
};
use clap::Parser;
use eyre::WrapErr;
use hubcaps::repositories::Repository;
use std::path::PathBuf;
use terminal_log_symbols::colored::SUCCESS_SYMBOL;

/// Create a new label.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct CreateArgs {
    /// The color of the label in hex notation (without the hash).
    #[clap(long, short)]
    pub color: Option<String>,

    /// The description of the label.
    #[clap(long, short)]
    pub description: Option<String>,

    /// The label definitions file to use for template resolution.
    #[clap(long, short)]
    pub file: Option<PathBuf>,

    /// The name of the label.
    #[clap(long, short)]
    pub name: String,

    /// The template to use for the new label.
    #[clap(conflicts_with("color"), long)]
    pub template: Option<String>,

    /// The GitHub personal access token. Takes precedence over environment
    /// variables.
    #[clap(long, short)]
    pub token: Option<String>,
}

impl CreateArgs {
    pub async fn run(self, _cli: Cli, repo: Repository, repo_raw: impl ToString) -> Result<()> {
        // If a template has been specified, take that color. If not, take the color
        // passed by the CLI. Either way, one or the other has to be specified.
        let label_definition_file =
            read_from_cli_arg_or_fallback_to_config_dir(self.file.as_deref())?;
        let color: Option<&str> = self
            .template
            .as_deref()
            .and_then(|template_name| {
                label_definition_file
                    .templates
                    .iter()
                    .find(|&v| v.name == template_name)
                    .map(|v| v.color.as_ref())
            })
            .or_else(|| self.color.as_deref());

        if color.is_none() {
            return Err(Error::NoTemplateOrColorSpecified).wrap_err_with(|| "You either have to specify a template name or a color. Make sure that the template does indeed exist inside the label definitions file.");
        }

        let label = JsonLabel::from(
            // Safety: above if statement.
            color.unwrap().to_string(),
            self.description.unwrap_or_else(|| "".into()),
            self.name,
        );
        let label_name = label.name.clone();

        let res = repo.labels().create(&label.into()).await;
        match res {
            Err(e) => {
                if e.is_label_already_exists_error() {
                    return Err(Error::LabelAlreadyExists(label_name)).wrap_err_with(|| {
                        "GitHub doesn't support multiple labels with the same name"
                    });
                } else if e.is_repo_not_found_error() {
                    return Err(Error::RepoNotFound(repo_raw.to_string())).wrap_err_with(|| {
                        "Make sure that the repository does exist before using the CLI"
                    });
                } else if e.is_user_unauthorized() {
                    return Err(Error::Unauthorized(repo_raw.to_string())).wrap_err_with(|| {
                        "Make sure that your personal access token has push access to the repository"
                    });
                }

                return Err(Error::Api(e)).wrap_err_with(|| {
                    "Something went wrong during label creation. Please try again."
                });
            }
            _ => {
                println!("{} Created label {:?}", SUCCESS_SYMBOL, label_name);
            }
        }

        Ok(())
    }
}
