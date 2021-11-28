use crate::{cli::Cli, file::read_from_cli_arg_or_fallback_to_config_dir, Result};
use clap::Parser;
use futures::stream::{self, StreamExt};
use hubcaps::repositories::Repository;
use std::path::PathBuf;
use terminal_log_symbols::colored::{ERROR_SYMBOL, INFO_SYMBOL};

/// Updates all labels from a label definition file.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct UpdateArgs {
    /// The label definitions file to use for updating.
    #[clap(long, short)]
    pub file: Option<PathBuf>,

    /// Delete all labels inside the repository.
    #[clap(long)]
    pub purge: bool,
}

impl UpdateArgs {
    pub async fn run(
        self,
        _cli: Cli,
        repo: Repository,
        concurrent_connections: usize,
    ) -> Result<()> {
        let label_definition_file = read_from_cli_arg_or_fallback_to_config_dir(self.file)?;
        let labels = label_definition_file.labels;

        // Fetch all labels that currently exist and filter them. Filtering is done by
        // comparing the repo labels to the one read from the file. The name,
        // description and color are taken into account.
        let existing_labels = repo.labels().iter().collect::<Vec<_>>().await;
        let (existing_labels, _errored_labels): (Vec<_>, Vec<_>) =
            existing_labels.into_iter().partition(|r| r.is_ok());

        let mut existing_labels: Vec<_> = existing_labels.into_iter().map(|r| r.unwrap()).collect();
        // let _errored_labels: Vec<_> = errored_labels.into_iter().map(|r|
        // r.unwrap_err()).collect();

        // Purge old labels.
        if self.purge {
            println!(
                "{} Purging all {} labels...",
                INFO_SYMBOL,
                existing_labels.len(),
            );

            let repo_labels = repo.labels();
            for label in existing_labels.iter() {
                repo_labels.delete(&label.name).await?;
            }

            // Set the existing labels to an empty vector. If not done than the code below
            // won't create the labels again.
            existing_labels.clear();
        }

        let (same_by_name_labels, labels_to_create): (Vec<_>, Vec<_>) = labels
            .into_iter()
            .partition(|lbl| existing_labels.iter().any(|v| lbl.name == v.name));
        let (_, labels_to_update): (Vec<_>, Vec<_>) = same_by_name_labels
            .into_iter()
            .partition(|lbl| existing_labels.iter().any(|v| lbl == v));

        println!(
            "{} Repository has {} labels, creating {} and updating {}...",
            INFO_SYMBOL,
            existing_labels.len(),
            labels_to_create.len(),
            labels_to_update.len(),
        );

        // Only create labels that are different from the ones that are already present.
        let fetches = stream::iter(labels_to_create.into_iter().map(|label| async {
            (
                label.name.clone(),
                repo.labels().create(&label.into()).await,
            )
        }))
        .buffer_unordered(concurrent_connections)
        .collect::<Vec<_>>();
        for (label_name, res) in fetches.await {
            if res.is_err() {
                eprintln!("{} Failed to create label {:?}", ERROR_SYMBOL, label_name);
            }
        }

        // TODO: can this be rewritten to not clone the name twice for error reporting?
        let fetches = stream::iter(labels_to_update.into_iter().map(|label| async {
            (
                label.name.clone(),
                repo.labels()
                    .update(&label.name.clone(), &label.into())
                    .await,
            )
        }))
        .buffer_unordered(concurrent_connections)
        .collect::<Vec<_>>();
        for (label_name, res) in fetches.await {
            if res.is_err() {
                eprintln!("{} Failed to create label {:?}", ERROR_SYMBOL, label_name);
            }
        }

        Ok(())
    }
}
