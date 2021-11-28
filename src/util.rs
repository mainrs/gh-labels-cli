use crate::error::Error;
use eyre::{Context, Result};
use hubcaps::{Credentials, Github};
use std::{borrow::Cow, env};
use url::Url;

const USER_AGENT: &str = "gh-labels-cli (https://github.com/mainrs/gh-labels-cli)";

fn github_api_token(cli_token: Option<&str>) -> Option<Cow<'_, str>> {
    println!("{:?}", cli_token.clone());
    cli_token.map(Into::into).or_else(|| {
        env::var("GH_LABELS_TOKEN")
            .or_else(|_| env::var("GITHUB_TOKEN"))
            .ok()
            .map(Into::into)
    })
}

pub fn create_github_api_client(cli_token: Option<&str>) -> Result<Github> {
    let token = github_api_token(cli_token);
    match token {
        Some(token) => Github::new(USER_AGENT, Credentials::Token(token.to_string())).wrap_err_with(|| "Failed to create GitHub API client"),
        None => Err(Error::NoTokenSpecified).wrap_err_with(|| "Make sure to either set the API token via the environment variables `GH_LABELS_TOKEN` or `GITHUB_TOKEN` or pass the token to the CLI via the `-t,--token` flag.")
    }
}

pub type GitHubRepo<'a> = (Cow<'a, str>, Cow<'a, str>);

/// Parses the repository CLI argument and constructs a GitHubRepo instance.
///
/// # Returns
///
/// `Ok(GitHubRepo)` if the argument had the right format,
/// `Err(Error::InvalidRepoFormat)` otherwise.
fn github_repo_from_cli_arg(arg: &str) -> std::result::Result<GitHubRepo<'_>, Error> {
    let number_of_slashes = arg.matches('/').count();

    match number_of_slashes {
        1 => {
            // Safety: match arm.
            let slash_index = arg.find('/').unwrap();
            Ok((arg[..slash_index].into(), arg[slash_index + 1..].into()))
        }
        _ => {
            // Parse the parameter as a URL. If it's valid and its host is
            // github.com, take the last two path segments and interpret them as
            // owner and repo. If not, return an error.
            let url = Url::parse(arg).map_err(|_| Error::InvalidRepoFormat)?;
            if url.host_str() == Some("github.com") {
                if let Some(mut segments) = url.path_segments() {
                    let repo = segments.nth_back(0).map(|v| match v.strip_suffix(".git") {
                        Some(without) => without,
                        None => v,
                    });
                    let owner = segments.nth_back(0);

                    if let Some(owner) = owner {
                        if let Some(repo) = repo {
                            return Ok((owner.to_string().into(), repo.to_string().into()));
                        }
                    }
                }
            }

            Err(Error::InvalidRepoFormat)
        }
    }
}

pub fn get_github_repo_and_owner(repo_arg: &str) -> Result<GitHubRepo<'_>> {
    github_repo_from_cli_arg(repo_arg).wrap_err_with(|| {
        "The repository has to be provided as `owner/repo` or as a https repository url!"
    })
}
