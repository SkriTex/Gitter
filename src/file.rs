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

    pub fn create_gitter_dir() {
        let env = env::current_dir().unwrap();
        let git_path = env.join(".git");
        let gitter_path = git_path.join("gitter");

        if fs::create_dir_all(&gitter_path).is_ok() {
            let settings_path = gitter_path.join("settings.txt");

            match File::create(&settings_path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "Add your settings here.") {
                        eprintln!("Failed to write to the file: {}", e);
                    } else {
                        println!("Created settings file: {}", settings_path.display());
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn read_settings(path: &PathBuf) -> HashMap<String, String> {
        let mut settings = String::new();

        if path.exists() {
            settings = fs::read_to_string(path)
                .map_err(|e| {
                    eprintln!("{}", e);
                })
                .unwrap_or_else(|_| String::new());
        }

        settings
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| line.contains('='))
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

        let mut sorted_keys: Vec<_> = self.settings.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            writeln!(file, "{}={}", key, self.settings[key]).expect(&format!(
                "Could not write '{}={}' in settings",
                key, self.settings[key]
            ));
        }
    }

    pub fn get_settings_value(&self, key: &str) -> &String {
        self.settings.get(key).unwrap()
    }

    pub fn set_settings_value(&mut self, key: &str, value: &str) -> Result<String, String> {
        if value.trim().is_empty() {
            return Err("Value cannot be empty".to_string());
        }

        let _ = &self.settings.insert(key.to_string(), value.to_string());

        self.write_settings();
        Ok(format!("Key '{}' updated.", key))
    }

    pub fn text_to_vec(path: &PathBuf) -> Vec<String> {
        let text = fs::read_to_string(path).unwrap();
        text.lines().map(|line| line.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_gitter_dir() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        FileHandle::create_gitter_dir();

        let gitter_path = temp_dir.path().join(".git/gitter");
        assert!(gitter_path.exists());
        assert!(gitter_path.is_dir());

        let settings_path = gitter_path.join("settings.txt");
        assert!(settings_path.exists());
        let content = fs::read_to_string(settings_path).unwrap();
        assert_eq!(content.trim(), "Add your settings here.");
    }

    #[test]
    fn test_write_settings() {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("settings.txt");

        let mut test_settings = HashMap::new();
        test_settings.insert("key1".to_string(), "value1".to_string());
        test_settings.insert("key2".to_string(), "value2".to_string());

        let file_handle = FileHandle {
            settings: test_settings,
            settings_path: temp_file.clone(),
            gitter_path: Path::new("").to_path_buf(),
        };

        file_handle.write_settings();

        let content =
            fs::read_to_string(&file_handle.settings_path).expect("Failed to read settings file");
        let expected_content = "key1=value1\nkey2=value2\n";
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_read_settings() {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("settings.txt");

        fs::write(&temp_file, "key1=value1\nkey2=value2\n").expect("Failed to write settings.");

        let mut test_settings = HashMap::new();
        test_settings.insert("key1".to_string(), "value1".to_string());
        test_settings.insert("key2".to_string(), "value2".to_string());

        let settings = FileHandle::read_settings(&temp_file);

        assert_eq!(test_settings, settings);
    }

    #[test]
    fn test_get_settings_value() {
        let mut settings = HashMap::new();
        settings.insert("key1".to_string(), "value1".to_string());

        let file_handle = FileHandle {
            settings: settings.clone(),
            gitter_path: PathBuf::new(),
            settings_path: PathBuf::new(),
        };

        let value = file_handle.get_settings_value("key1");

        assert_eq!("value1", value);
    }

    #[test]
    fn test_set_settings_value() {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("settings.txt");

        let mut test_settings = HashMap::new();

        let mut file_handle = FileHandle {
            settings: test_settings.clone(),
            settings_path: temp_file,
            gitter_path: PathBuf::new(),
        };

        test_settings.insert("key1".to_string(), "value1".to_string());

        let result = file_handle.set_settings_value("key1", "value1");

        match result {
            Err(e) => assert_eq!("Value cannot be empty", e),
            Ok(o) => assert_eq!("Key 'key1' updated.", o),
        }

        assert_eq!(test_settings, file_handle.settings);
    }

    #[test]
    fn test_text_to_vec() {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("settings.txt");

        fs::write(&temp_file, "key1=value1\nkey2=value2\n").expect("Failed to write settings file.");

        let result: Vec<String> = FileHandle::text_to_vec(&temp_file);
        let expected: Vec<String> = vec!["key1=value1".to_string(), "key2=value2".to_string()];

        assert_eq!(expected, result);
    }
}
