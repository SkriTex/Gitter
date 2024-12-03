use clap::{ArgGroup, Parser, Subcommand};

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
    #[command(group = ArgGroup::new("run_options").required(false))]
    Run {
        #[arg(name = "name")]
        name: Option<String>,
        #[arg(value_parser = parse_key_val, trailing_var_arg = true)]
        args: Vec<(String, String)>,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let mut parts = s.splitn(2, '=');
    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
        Ok((key.to_string(), value.to_string()))
    } else {
        Err(format!("Invalid key-value pair: {}", s))
    }
}
