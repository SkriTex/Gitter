use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'p', long = "path", default_value = ".")]
    path: Option<String>,

    #[arg(short = 'f', long = "feature")]
    feature_branch: Option<String>,

    #[arg(short = 'l', long = "local")]
    local_branch: Option<String>,

    #[arg(long = "pull")]
    pull: bool,

    #[arg(long = "push")]
    push: bool,
}

fn main() {
    let cli = Cli::parse();

    if let (Some(path), Some(feature_branch), Some(local_branch)) =
        (&cli.path, &cli.feature_branch, &cli.local_branch)
    {
        if cli.pull {
            git_command("checkout", feature_branch, path);
            git_command("pull", feature_branch, path);
            git_command("checkout", local_branch, path);
        }

        git_command("merge", feature_branch, path);

        if cli.push {
            git_command("push", local_branch, path);
        }

        println!("Finished...");
    }
}

fn git_command(command: &str, branch: &str, path: &str) {
    let args = match command {
        "push" => vec!["push", "origin", branch],
        "merge" => vec!["merge", branch],
        "checkout" => vec!["checkout", branch],
        "pull" => vec!["pull", "origin", branch],
        _ => {
            return;
        }
    };

    let output = Command::new("git")
        .args(&args)
        .current_dir(path)
        .output()
        .expect("failed to execute {command} command");

    print_output(&output);
}

fn print_output(output: &Output) {
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
