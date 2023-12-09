use config::{Config, File};
use dirs::home_dir;
use rand::Rng;
use std::os::unix;
use std::path::PathBuf;
use std::{fs, usize};

fn main() {
    let settings = load_config();
    let number = get_number(get_config_value(&settings, "number"));
    let origin = expand_path(get_config_value(&settings, "origin"));
    let target = expand_path(get_config_value(&settings, "target"));

    match (origin, target) {
        (Some(ref origin_path), Some(ref target_path)) => {
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
        _ => println!("No paths provided"),
    }
}
fn expand_path(path_option: Option<String>) -> Option<PathBuf> {
    path_option
        .map(|path_str| {
            // 使用map对Option进行转换操作
            if path_str.starts_with("~/") {
                home_dir().map(|home| home.join(path_str.trim_start_matches("~/")))
            // 如果以"~/"开头，则使用home_dir()获取用户目录并拼接路径
            } else {
                Some(PathBuf::from(path_str)) // 否则直接将字符串转换为PathBuf类型
            }
        })
        .unwrap_or_else(|| None) // unwrap_or_else处理Option为None的情况，提供一个默认的返回值
}

fn create_dir(path: &PathBuf) {
    fs::create_dir_all(path).unwrap();
}
// fn get_config_number(settings: &Config, key: &str) -> Option<String> {
//     settings
//         .get(format!("configurations.{}", key).as_str())
//         .ok()
// }

fn get_number(number: Option<String>) -> usize {
    number
        .and_then(|num| num.parse::<usize>().ok())
        .unwrap_or(1)
}

fn get_config_value(settings: &Config, key: &str) -> Option<String> {
    settings
        .get(format!("configurations.{}", key).as_str())
        .ok()
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
