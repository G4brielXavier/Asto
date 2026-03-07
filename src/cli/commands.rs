use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {

    // asto export --path <PATH> --json OR --md --silent
    Export {

        // --path -p: Asto file path
        #[arg(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        path: Option<PathBuf>,

        // --json -j: JSON export
        #[arg(short, long)]
        json: bool,

        // --md -m: Markdown export
        #[arg(short, long)]
        md: bool,

        // --silent -s: Disable logs
        #[arg(short, long)]
        silent: bool

    }

}