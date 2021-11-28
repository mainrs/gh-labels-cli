use crate::{cli::Cli, Result};
use clap::Parser;
use eyre::{eyre, WrapErr};
use std::process::Command;
use terminal_link::Link;

/// Install `gh` CLI integrations.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct IntegrationArgs {
    #[clap(subcommand)]
    pub cmd: IntegrationSubCommand,
}

#[derive(Clone, Debug, Parser)]
pub enum IntegrationSubCommand {
    Install,
    Uninstall,
}

impl IntegrationArgs {
    fn create_new_alias(&self, name: &str, cmd: &str) -> Result<()> {
        let exit_code = Command::new("gh")
            .args(&["alias", "set", name, "-s", cmd])
            .status()
            .wrap_err_with(|| {
                format!(
                    "Make sure that the official GitHub CLI is installed: {}",
                    Link::new("Website", "https://cli.github.com")
                )
            })?;
        if exit_code.success() {
            Ok(())
        } else {
            Err(eyre!(
                "Calling `gh` CLI tool failed with a non-zero exit code"
            ))
        }
    }

    fn remove_alias(&self, alias: &str) -> Result<()> {
        let exit_code = Command::new("gh")
            .args(&["alias", "delete", alias])
            .status()
            .wrap_err_with(|| {
                format!(
                    "Make sure that the official GitHub CLI is installed: {}",
                    Link::new("Website", "https://cli.github.com")
                )
            })?;
        if exit_code.success() {
            Ok(())
        } else {
            Err(eyre!(
                "Calling `gh` CLI tool failed with a non-zero exit code"
            ))
        }
    }

    fn add_aliases(&self) -> Result<()> {
        // gh alias set labels -s 'gh-labels api $@'
        self.create_new_alias("labels", "/home/me/.cargo/bin/gh-labels api $@")?;

        // gh alias set -s new 'gh repo create $1; cd $1; gh labels update --purge'
        self.create_new_alias("new", "gh repo create $1; cd $1; gh labels $@ update --purge")
    }

    fn remove_aliases(&self) -> Result<()> {
        self.remove_alias("labels")?;
        self.remove_alias("new")
    }

    pub fn run(self, _cli: Cli) -> Result<()> {
        match self.cmd {
            IntegrationSubCommand::Install => {
                self.remove_aliases()?;
                self.add_aliases()?;
            }
            IntegrationSubCommand::Uninstall => {
                self.remove_aliases()?;
            }
        }

        Ok(())
    }
}
