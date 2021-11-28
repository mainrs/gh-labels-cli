use crate::{cli::Cli, config::config_file, Result};
use clap::Parser;
use edit::edit;
use eyre::{eyre, Context};
use std::fs;

/// Query or edit the global configuration.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct ConfigArgs {
    // Edit the file in your default editor.
    #[clap(conflicts_with("path"), conflicts_with("read"), long)]
    pub edit: bool,

    /// Read the configuration file and print it to standard output.
    #[clap(conflicts_with("edit"), conflicts_with("path"), long)]
    pub read: bool,

    /// Print the path to the configuration file to standard output.
    #[clap(conflicts_with("edit"), conflicts_with("read"), long)]
    pub path: bool,
}

impl ConfigArgs {
    pub fn run(self, _cli: Cli) -> Result<()> {
        match config_file() {
            Some(path) => {
                if self.edit {
                    let mut content = String::new();

                    // If the file exists, read the file in and populate the user's editor with its
                    // content.
                    if path.exists() {
                        content = fs::read_to_string(&path)?;
                    }
                    let content = edit(content).wrap_err_with(|| {
                        "An error occurred while editing your configuration file"
                    })?;

                    // Write edited content back. In the case the file does not exist, all parent
                    // directories are created beforehand to ensure that the write is successfull.
                    if let Some(dir) = &path.parent() {
                        fs::create_dir_all(dir)?;
                    }

                    fs::write(path, content)
                        .wrap_err_with(|| "Failed to write edited configuration to file")?;
                } else if self.path {
                    println!("{}", path.display());
                } else if self.read {
                    println!(
                        "{}",
                        fs::read_to_string(path)
                            .wrap_err_with(|| "The configuration file does not exist")?
                    );
                }

                Ok(())
            }
            None => Err(eyre!("No configuration file path could be resolved")),
        }
    }
}
