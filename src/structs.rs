use clap::{ArgGroup, Parser, Subcommand};

pub enum GitCommand {
    Checkout,
    Pull,
    Push,
    Commit,
    Merge,
    Add,
}

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

    Commit {
        #[arg(short = 'p', long = "path", default_value = ".")]
        path: Option<String>,
        #[arg(short = 'f', long = "feature")]
        feature_branch: Option<String>,
        #[arg(short = 'l', long = "local")]
        local_branch: Option<String>,
        #[arg(short = 'm', long = "message")]
        message: Option<String>,
        #[arg(long = "pull")]
        pull: bool,
        #[arg(long = "push")]
        push: bool,
    },

    //Command used for showing settings
    //[all] displays all settings
    //[key] display value of key from settings
    #[command(group(
        ArgGroup::new("show_options")
        .args(&["all", "key"])
        .multiple(false)
    ))]
    Show {
        #[arg(short = 'a', long = "all", group = "show_options")]
        all: bool,
        #[arg(short = 'k', long = "key", group = "show_options")]
        key: Option<String>,
    },

    //Used for running custom automation files
    Run {},
}

pub struct Settings {
    pub feature_branch: String,
    pub local_branch: String,
}
