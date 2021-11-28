use clap::Parser;

mod api;
mod config;
mod integration;

#[derive(Clone, Debug, Parser)]
pub enum SubCommand {
    Api(api::ApiArgs),
    Config(config::ConfigArgs),
    Integration(integration::IntegrationArgs),
}

#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}
