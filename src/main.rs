use link_creater::*;
use rand::Rng;
use std::{fs, usize};

fn main() {
    let config_path = "~/.config/link_creater_config.toml";

    let settings = load_config(config_path);
    let number = get_number(&settings, "number");
    let origin = expand_path(&settings, "origin");
    let target = expand_path(&settings, "target");

    match (origin, target) {
        (Some(ref origin_path), Some(ref target_path)) => {
            fs::create_dir_all(origin_path).unwrap();
            fs::create_dir_all(target_path).unwrap();
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
