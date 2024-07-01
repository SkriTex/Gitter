use clap::{Parser, Subcommand};

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
}

pub struct Settings {
    pub feature_branch: String,
    pub local_branch: String,
}
