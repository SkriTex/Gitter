use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub struct FileHandle {
    gitter_path: PathBuf,
    settings_path: PathBuf,
    settings: HashMap<String, String>,
}
impl FileHandle {
    pub fn new() -> Self {
        let env_path = env::current_dir().unwrap();
        let git_path = env_path.join(".git");
        let gitter_path = FileHandle::get_gitter_dir(&git_path);
        let settings_path = FileHandle::get_settings_path(&gitter_path);
        let settings = Self::read_settings(&settings_path);

        FileHandle {
            gitter_path,
            settings_path,
            settings,
        }
    }

    fn get_gitter_dir(path: &Path) -> PathBuf {
        let gitter_path = path.join("gitter");

        if gitter_path.exists() && gitter_path.is_dir() {
            return gitter_path;
        }
        eprintln!(
            "Could not get 'gitter' directory. {}",
            gitter_path.display()
        );

        gitter_path
    }

    fn get_settings_path(path: &Path) -> PathBuf {
        let settings_path = path.join("settings.txt");
        return settings_path;
    }

    pub fn get_gitter_path(&self) -> &PathBuf {
        &self.gitter_path
    }

    pub fn get_settings(&self) -> &HashMap<String, String> {
        &self.settings
    }

    pub fn create_gitter_dir(&self) {
        if fs::create_dir_all(&self.gitter_path).is_ok() {
            match File::create(&self.settings_path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "Add your settings here.") {
                        eprintln!("Failed to write to the file: {}", e);
                    } else {
                        println!("Created settings file: {}", self.settings_path.display());
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn read_settings(path: &PathBuf) -> HashMap<String, String> {
        let mut settings = String::new();

        if !Path::new(path).exists() {
            settings = fs::read_to_string(path).expect("Failed to read settings file!");
        }

        settings
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| line.contains('*'))
            .map(|line| {
                let mut parts = line.splitn(2, '=');
                let key = parts.next().unwrap_or("").trim().to_string();
                let value = parts.next().unwrap_or("").trim().to_string();
                (key, value)
            })
            .collect()
    }

    fn write_settings(&self) {
        let mut file = File::create(&self.settings_path).expect("Could not open settings file.");

        for (key, value) in &self.settings {
            writeln!(file, "{}={}", key, value)
                .expect(&format!("Could not write '{}={}' in settings", key, value));
        }
    }

    pub fn get_settings_value(&self, key: &str) -> &String {
        self.settings.get(key).unwrap()
    }

    pub fn set_settings_value(&mut self, key: &str, value: &str) -> Result<String, String> {
        if value.trim().is_empty() {
            return Err("Value cannot be empty".to_string());
        }

        let old = self
            .settings
            .insert(key.to_string(), value.to_string())
            .unwrap();

        self.write_settings();
        Ok(format!("Value '{}' replaced with '{}'.", old, value))
    }

    pub fn text_to_vec(path: &PathBuf) -> Vec<String> {
        let text = fs::read_to_string(path).unwrap();
        text.lines().map(|line| line.to_string()).collect()
    }
}
