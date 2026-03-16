mod cli;
mod core;

use clap::Parser;

use cli::args::Args;
use cli::matches::matches_command;


fn main() {

    let cli: Args = Args::parse();
    matches_command(&cli.command);

}
