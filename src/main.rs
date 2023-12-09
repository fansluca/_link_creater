use config::{Config, File};
use dirs::home_dir;
use rand::Rng;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::{fs, usize};

fn expand_path(path_option: Option<String>) -> Option<PathBuf> {
    match (path_option, home_dir()) {
        (Some(path_str), Some(home)) if path_str.starts_with("~/") => {
            Some(home.join(path_str.trim_start_matches("~/")))
        }
        (Some(path_str), _) => Some(PathBuf::from(path_str)),
        _ => None,
    }
}
fn create_dir(path: &PathBuf) {
    if Path::new(path).is_dir() {
        println!("directory exists");
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => println!("Folder '{}' created successfully.", path.display()),
            Err(e) => eprintln!("Error creating folder '{}': {}", path.display(), e),
        }
    }
}
fn get_number(number: Option<String>) -> usize {
    number
        .map(|num| {
            num.parse::<usize>().unwrap_or_else(|_| {
                eprintln!("Invalid value for 'configurations.number'");
                1
            })
        })
        .unwrap_or_else(|| {
            eprintln!("No 'configurations.number' provided");
            1
        })
}
fn get_config_value(settings: &Config, key: &str) -> Option<String> {
    settings
        .get(format!("configurations.{}", key).as_str())
        .ok()
}

fn main() {
    let settings_result = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build();
    let settings = match settings_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let number = get_number(get_config_value(&settings, "number"));
    let origin = expand_path(get_config_value(&settings, "origin"));
    let target = expand_path(get_config_value(&settings, "target"));

    match (origin, target) {
        (Some(ref origin_path), Some(ref target_path)) => {
            // println!("origin: {}", origin_path.display());
            // println!("target: {}", target_path.display());

            create_dir(target_path);
            for entry in fs::read_dir(target_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(&path).unwrap();
                }
                println!("removed {}", path.display());
            }

            create_dir(origin_path);
            let dir = fs::read_dir(origin_path).unwrap();

            let file_names_string: Vec<String> = dir
                .filter_map(|entry| {
                    entry
                        .ok()
                        .map(|e| e.file_name().to_string_lossy().to_string())
                })
                .collect();
            let mut selected_files: Vec<&String> = Vec::with_capacity(number);
            for (i, name) in file_names_string.iter().enumerate() {
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
                let source_file = &origin_path.join(name);
                let link_name = &target_path.join(name);
                match unix::fs::symlink(source_file, link_name) {
                    Ok(_) => println!("symlinked {}", link_name.display()),
                    Err(e) => eprintln!(
                        "Error symlinking {} to {}: {}",
                        source_file.display(),
                        link_name.display(),
                        e
                    ),
                };
            }
        }
        (Some(ref origin_path), None) => {
            println!("origin: {}", origin_path.display());
            eprintln!("No target path provided");

            create_dir(origin_path);
        }
        (None, Some(ref target_path)) => {
            println!("No origin path provided");
            println!("target: {}", target_path.display());

            create_dir(target_path);
        }
        (None, None) => {
            eprintln!("No origin and target paths provided");
        }
    }
}
