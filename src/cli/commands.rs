use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {

    // asto export --path <PATH> --json OR --md --silent
    #[command(about="Converts a Asto file to json or markdown")]
    Export {

        // --path -p: Asto file path
        #[arg(value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
        path: Option<PathBuf>,

        // --json -j: JSON export
        #[arg(short, long, value_name = "JSON")]
        json: bool,

        // --md -m: Markdown export
        #[arg(short, long, value_name = "MARKDOWN")]
        md: bool,

        // --silent -s: Disable logs
        #[arg(short, long, value_name = "SILENT")]
        silent: bool

    },

    #[command(about="Makes a analize over Asto file and verify and returns informations about Command's Version")]
    Version {

        #[arg(value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
        path: Option<PathBuf>,
    
    },

    #[command(about="Makes a analize over Asto file and verify if each command has what is necessary")]
    Status {

        #[arg(value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
        path: Option<PathBuf>,
    
    },    

    #[command(about="Makes a Tree with all commands and your informations.")]
    Tree {

        #[arg(value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
        path: Option<PathBuf>,
    
    }

}