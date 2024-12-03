use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::file::FileHandle;

pub struct Task {
    task_name: String,
    task_path: PathBuf,
    settings: HashMap<String, String>,
}

impl Task {
    pub fn new(task_name: String, gitter_path: PathBuf, settings: HashMap<String, String>) -> Task {
        let task_path: PathBuf = gitter_path.join(task_name.clone() + ".txt");
        Task {
            task_name,
            task_path,
            settings,
        }
    }

    pub fn run_task(&self) {
        if self.task_path.exists() {
            let lines: Vec<String> = FileHandle::text_to_vec(&self.task_path);

            if lines.is_empty() {
                eprintln!("Task {} has no rules in it.", &self.task_name);
            }

            for mut command in lines {
                if command.trim().is_empty() {
                    continue;
                }

                command = self.replace_settings(command);
                println!("{command}");
                // let child = Command::new("git")
                //     .stdin(Stdio::piped())
                //     .stdout(Stdio::piped())
                //     .stderr(Stdio::piped())
                //     .args(command.split_whitespace())
                //     .spawn()
                //     .expect("Failed to spawn child process");
                //
                // let output = child.wait_with_output().expect("Failed to read stdout.");
                // println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        } else {
            eprintln!(
                "Task {} on path {} does not exist.",
                self.task_name,
                self.task_path.display()
            );
        }
    }

    fn replace_settings(&self, mut command: String) -> String {
        for (key, value) in &self.settings {
            let placeholder = format!("${{{}}}", key);
            command = command.replace(&placeholder, value);
        }

        command
    }
}
