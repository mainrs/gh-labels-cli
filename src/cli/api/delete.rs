use clap::{AppSettings, Clap};

/// Create a new label.
#[derive(Clap, Debug)]
#[clap(author, setting(AppSettings::ColoredHelp), version)]
pub struct DeleteArgs {
    /// The name of the label.
    #[clap(long, short)]
    pub name: String,
}
