mod structs;

use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs};
use structs::Cli;
use structs::Commands;
use structs::GitCommand;
use structs::Settings;

fn main() {
    let cli = Cli::parse();
    let settings_content: Vec<String> = check_for_settings();
    let settings: Settings = parse_settings(&settings_content).expect("Failed to parse settings!");

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

        Commands::Run {} => {}

        Commands::Init {} => match env::current_dir() {
            Ok(path) => create_gitter_dir(&path),
            Err(e) => println!("{}", e),
        },

        Commands::Set { key, value } => {
            let settings: Vec<String> = replace_settings_value(settings_content, key, value);
            match write_settings(&settings) {
                Ok(_) => println!("Settings saved."),
                Err(e) => eprintln!("Failed to write settings: {}", e),
            }
        }

        Commands::Show { all, key } => {
            if *all {
                for x in settings_content.iter() {
                    println!("{}", x);
                }
            }

            if let Some(key) = key.as_ref() {
                println!("{}", find_value_from_key(&settings_content, key));
            }
        }
    }
}

fn create_gitter_dir(path: &PathBuf) {
    let git_path = path.join(".git");

    if git_path.exists() && git_path.is_dir() {
        let gitter_path = git_path.join("gitter");

        if fs::create_dir_all(&gitter_path).is_ok() {
            let file_path = gitter_path.join("settings.txt");

            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "Add your settings here") {
                        eprintln!("Failed to write to the file: {}", e);
                    } else {
                        println!("Created and wrote to file: {}", file_path.display());
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    } else {
        println!(".git folder does not exist, please run git init firt.");
    }
}

fn find_value_from_key(settings: &Vec<String>, key: &str) -> String {
    let prefix = format!("{}=", key);

    for entry in settings.iter() {
        if entry.starts_with(&prefix) {
            return entry.to_string();
        }
    }

    "".to_string()
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
    let caller_path = env::current_dir().unwrap();
    let repo_settings = caller_path.join(".git").join("gitter").join("settings.txt");
    let cwd = env::current_exe().expect("CWD not found!");
    let parent = cwd.parent().expect("Failed to extract parent dir!");
    let settings_path = parent.join("gitter_settings.txt");
    let mut settings = String::new();

    if !Path::new(&repo_settings).exists() {
        settings = fs::read_to_string(repo_settings).expect("Failed to read settings file!");
        // _ = fs::File::create(&repo_settings);
    }
    // } else {
    //     settings = fs::read_to_string(settings_path).expect("Failed to read settings file!");
    // }

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
