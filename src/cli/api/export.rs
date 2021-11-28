use crate::{
    cli::Cli,
    file::{JsonFile, JsonLabel},
    Result,
};
use clap::Parser;
use hubcaps::repositories::Repository;
use std::fs;
use tokio::stream::StreamExt;

/// Export a repository's labels into a file.
#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct ExportArgs {
    /// The file to write the labels to.
    #[clap(long, short)]
    pub file: String,
}

impl ExportArgs {
    pub async fn run(self, _cli: Cli, repo: Repository) -> Result<()> {
        let labels = repo.labels().iter().collect::<Vec<_>>().await;
        let (labels, _errored_labels): (Vec<_>, Vec<_>) =
            labels.into_iter().partition(|r| r.is_ok());

        let json_labels: Vec<JsonLabel> = labels
            .into_iter()
            .map(|v| v.unwrap())
            .map(Into::into)
            .collect();

        let json_file = JsonFile {
            labels: json_labels,
            ..Default::default()
        };

        fs::write(self.file, serde_json::to_string(&json_file)?)?;
        Ok(())
    }
}
