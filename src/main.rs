mod file;
mod structs;
mod task;

use clap::Parser;
use file::FileHandle;
use structs::Cli;
use structs::Commands;
use task::Task;

fn main() {
    let cli = Cli::parse();
    let mut file_handle = FileHandle::new();

    match &cli.command {
        Commands::Init {} => {
            file_handle.create_gitter_dir();
        }

        Commands::Run { name } => {
            let task_name = name.as_ref().unwrap().to_string();
            let task = Task::new(task_name, file_handle.get_gitter_path().clone());
            task.run_task();
        }

        Commands::Show { all, key } => {
            if *all {
                for (k, v) in file_handle.get_settings().iter() {
                    println!("{}={}", k, v);
                }
            } else if let Some(key) = key.as_ref() {
                println!("{}", file_handle.get_settings_value(&key));
            }
        }

        Commands::Set { key, value } => {
            let result: Result<String, String> = file_handle.set_settings_value(key, value);

            match result {
                Err(e) => eprintln!("{}", e),
                Ok(o) => println!("{}", o),
            }
        }
    }
}
