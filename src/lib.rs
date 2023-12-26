use config::{Config, File};
use dirs::home_dir;
use std::os::unix;
use std::path::PathBuf;
use std::{fs, usize};

pub fn get_number(settings: &Config, key: &str) -> usize {
    let settings = match settings.get_string(format!("configurations.{}", key).as_str()) {
        Ok(n) => n,
        Err(e) => panic!("Problem load configuration: {:?}", e),
    };
    settings.parse::<usize>().unwrap()
}
pub fn expand_path(settings: &Config, key: &str) -> Option<PathBuf> {
    settings
        .get(format!("configurations.{}", key).as_str())
        .ok()
        .and_then(|path_str| {
            let path_str: String = path_str;
            if path_str.starts_with("~/") {
                home_dir().map(|home| home.join(path_str.trim_start_matches("~/")))
            } else {
                Some(PathBuf::from(path_str))
            }
        })
}
pub fn load_config(config_path: &str) -> Config {
    let expanded_config_path = home_dir()
        .map(|home| home.join(config_path.trim_start_matches("~/")))
        .expect("Failed to get home directory")
        .to_string_lossy()
        .into_owned();
    match Config::builder()
        .add_source(File::with_name(&expanded_config_path))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    }
}
pub fn symlink_file(origin_path: &PathBuf, target_path: &PathBuf, file_name: &str) {
    let source_file = PathBuf::from(origin_path).join(file_name);
    let link_name = PathBuf::from(target_path).join(file_name);
    match unix::fs::symlink(&source_file, &link_name) {
        Ok(_) => println!("symlinked {}", link_name.display()),
        Err(e) => eprintln!(
            "Error symlinking {} to {}: {}",
            source_file.display(),
            link_name.display(),
            e
        ),
    };
}
pub fn remove_file(path: &PathBuf) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path).unwrap();
        }
        println!("removed {}", path.display());
    }
}
pub fn get_file_names(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().to_string_lossy().to_string())
        })
        .collect()
}
