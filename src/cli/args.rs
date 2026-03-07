use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser, Debug)]
#[command(author = "dotxavket")]
#[command(version = "v1.0.0")]
#[command(about = "A domain-specific language for organizing and documenting CLI commands")]
#[command(long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands
}