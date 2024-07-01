mod structs;

use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};
use std::{env, fs};
use structs::Cli;
use structs::Commands;
use structs::GitCommand;
use structs::Settings;

fn main() {
    let settings_content: Vec<String> = check_for_settings();
    let settings: Settings = parse_settings(&settings_content).expect("Failed to parse settings!");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit {
            path,
            feature_branch,
            local_branch,
            message,
            pull,
            push,
        } => {
            let path = path.as_ref().unwrap();
            let feature_b = feature_branch
                .as_ref()
                .unwrap_or_else(|| &settings.feature_branch);
            let local_b = local_branch
                .as_ref()
                .unwrap_or_else(|| &settings.local_branch);

            if let Some(message) = message.as_ref() {
                if !message.trim().is_empty() {
                    git_commit(message, path);
                }
            }

            if *pull {
                git_command(GitCommand::Checkout, feature_b, path);
                git_command(GitCommand::Pull, feature_b, path);
                git_command(GitCommand::Checkout, local_b, path);
                git_command(GitCommand::Merge, feature_b, path);
            }

            if *push {
                git_command(GitCommand::Push, local_b, path);
            }
        }

        Commands::Set { key, value } => {
            let settings: Vec<String> = replace_settings_value(settings_content, key, value);
            match write_settings(&settings) {
                Ok(_) => println!("Settings saved."),
                Err(e) => eprintln!("Failed to write settings: {}", e),
            }
        }
    }
}

fn replace_settings_value(mut settings: Vec<String>, key: &str, value: &str) -> Vec<String> {
    let prefix = format!("{}=", key);
    let mut found = false;

    for entry in settings.iter_mut() {
        if entry.starts_with(&prefix) {
            *entry = format!("{}{}", prefix, value);
            found = true;
            break;
        }
    }

    if !found {
        settings.push(format!("{}{}", prefix, value));
    }

    settings
}

fn write_settings(settings: &[String]) -> io::Result<()> {
    let cwd = env::current_exe().expect("CWD not found!");
    let parent = cwd.parent().expect("Failed to extract parent dir!");
    let settings_path = parent.join("gitter_settings.txt");

    let mut file = fs::File::create(settings_path)?;

    for line in settings {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn check_for_settings() -> Vec<String> {
    let cwd = env::current_exe().expect("CWD not found!");
    let parent = cwd.parent().expect("Failed to extract parent dir!");
    let settings_path = parent.join("gitter_settings.txt");
    let mut settings = String::new();

    if !Path::new(&settings_path).exists() {
        _ = fs::File::create(&settings_path);
    } else {
        settings = fs::read_to_string(settings_path).expect("Failed to read settings file!");
    }

    settings.lines().map(|line| line.to_string()).collect()
}

fn parse_settings(content: &Vec<String>) -> Result<Settings, io::Error> {
    let mut feature_branch = String::new();
    let mut local_branch = String::new();

    for line in content {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            match parts[0].trim() {
                "feature_branch" => feature_branch = parts[1].trim().to_string(),
                "local_branch" => local_branch = parts[1].trim().to_string(),
                _ => {}
            }
        }

        // match parts.next() {
        //     Some("feature_branch") => feature_branch = parts.next().unwrap_or_default().to_string(),
        //     Some("local_branch") => local_branch = parts.next().unwrap_or_default().to_string(),
        //     _ => {}
        // }
    }

    Ok(Settings {
        feature_branch,
        local_branch,
    })
}

fn git_commit(message: &str, path: &str) {
    let commit_args = vec!["commit", "-m", message];

    git_command(GitCommand::Add, "", path);

    let output = Command::new("git")
        .args(&commit_args)
        .current_dir(path)
        .output()
        .expect("Failed to merge");

    print_output(&output);
}

fn git_command(command: structs::GitCommand, branch: &str, path: &str) {
    let args = match command {
        GitCommand::Push => vec!["push", "origin", branch],
        GitCommand::Merge => vec!["merge", branch],
        GitCommand::Checkout => vec!["checkout", branch],
        GitCommand::Pull => vec!["pull", "origin", branch],
        GitCommand::Add => vec!["add", "."],
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
