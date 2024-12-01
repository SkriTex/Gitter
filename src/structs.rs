use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,

    Set {
        #[arg(short, long, required = true)]
        key: String,
        #[arg(short, long, required = true)]
        value: String,
    },

    Show {
        #[arg(short = 'a', long = "all", group = "show_options")]
        all: bool,
        #[arg(short = 'k', long = "key", group = "show_options")]
        key: Option<String>,
    },

    //Used for running custom automation files
    Run {
        #[arg(name = "name", group = "run_options")]
        name: Option<String>,
    },
}
