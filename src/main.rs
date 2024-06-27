use std::io::{self, Write};
use std::process::{Command, Output};

fn main() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let args: Vec<&str> = input.split_whitespace().collect();

    let path = args[0];
    let feature = args[1];
    let local = args[2];

    if args.iter().any(|a| a.to_string() == "-p") {
        git_command("checkout", feature, path);
        git_command("pull", feature, path);
        git_command("checkout", local, path);
    }

    git_command("merge", feature, path);

    if args.iter().any(|a| a.to_string() == "-P") {
        git_command("push", local, path);
    }

    io::stdin().read_line(&mut String::new()).unwrap();
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
