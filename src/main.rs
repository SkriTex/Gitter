use clap::Parser;
use colored::Colorize;
use std::process::{Command, Output};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'p', long = "path", default_value = ".")]
    path: Option<String>,

    #[arg(short = 'f', long = "feature", required = true)]
    feature_branch: Option<String>,

    #[arg(short = 'l', long = "local", required = true)]
    local_branch: Option<String>,

    #[arg(short = 'm', long = "message")]
    message: Option<String>,

    #[arg(long = "pull")]
    pull: bool,

    #[arg(long = "push")]
    push: bool,
}

fn main() {
    let cli = Cli::parse();

    if let (Some(mess), Some(path)) = (&cli.message, &cli.path) {
        git_commit(mess, path);
    }

    if let (Some(path), Some(feature_branch), Some(local_branch)) =
        (&cli.path, &cli.feature_branch, &cli.local_branch)
    {
        if cli.pull {
            git_command("checkout", feature_branch, path);
            git_command("pull", feature_branch, path);
            git_command("checkout", local_branch, path);
            git_command("merge", feature_branch, path);
        }

        if cli.push {
            git_command("push", local_branch, path);
        }

        println!("{}", "Finished...".green());
    }
}

fn git_commit(message: &str, path: &str) {
    let commit_args = vec!["commit", "-m", message];

    git_command("add", "", path);

    let output = Command::new("git")
        .args(&commit_args)
        .current_dir(path)
        .output()
        .expect("Failed to merge");

    print_output(&output);
}

fn git_command(command: &str, branch: &str, path: &str) {
    let args = match command {
        "push" => vec!["push", "origin", branch],
        "merge" => vec!["merge", branch],
        "checkout" => vec!["checkout", branch],
        "pull" => vec!["pull", "origin", branch],
        "add" => vec!["add", "."],
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
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout).green());
    }
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}
