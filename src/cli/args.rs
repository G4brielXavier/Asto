use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser, Debug)]
#[command(author = "dotxavket")]
#[command(version = "v0.1.1")]
#[command(about = "Asto is a small but powerfull DSL designed to help CLI engineers organize, describe, and document command-line tools - clearly and efficiently.")]
#[command(long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands
}