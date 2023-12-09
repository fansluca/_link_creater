use config::{Config, File};
use dirs::home_dir;
use rand::Rng;
use std::os::unix;
use std::path::PathBuf;
use std::{fs, usize};

fn main() {
    let settings = load_config();
    let number = get_number(&settings, "number");
    let origin = expand_path(&settings, "origin");
    let target = expand_path(&settings, "target");

    match (origin, target) {
        (Some(ref origin_path), Some(ref target_path)) => {
            fs::create_dir_all(target_path).unwrap();
            fs::create_dir_all(origin_path).unwrap();
            remove_file(target_path);
            let origin_file = get_file_names(origin_path);

            let mut selected_files = Vec::with_capacity(number);

            for (i, name) in origin_file.iter().enumerate() {
                if i < number {
                    selected_files.push(name);
                } else {
                    let j = rand::thread_rng().gen::<usize>() % (i + 1);
                    if j < number {
                        selected_files[j] = name;
                    }
                }
            }
            for name in selected_files {
                symlink_file(origin_path, target_path, name);
            }
        }
        _ => println!("No paths provided"),
    }
}

fn get_number(settings: &Config, key: &str) -> usize {
    let settings = match settings.get_string(format!("configurations.{}", key).as_str()) {
        Ok(n) => n,
        Err(e) => panic!("Problem load configuration: {:?}", e),
    };
    settings.parse::<usize>().unwrap()
}
fn expand_path(settings: &Config, key: &str) -> Option<PathBuf> {
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
fn load_config() -> Config {
    match Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    }
}
fn symlink_file(origin_path: &PathBuf, target_path: &PathBuf, file_name: &str) {
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
fn remove_file(path: &PathBuf) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path).unwrap();
        }
        println!("removed {}", path.display());
    }
}
fn get_file_names(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().to_string_lossy().to_string())
        })
        .collect()
}
