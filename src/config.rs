use std::path::PathBuf;
use std::fs;
use std::env;
use std::io::BufReader;
use std::io::BufRead;

pub fn get_paths_from_config_in_home_folder() -> Vec<String> {
    let home_dir = env::home_dir().expect("No home directory found!");
    load_config_file(&home_dir).unwrap_or_else(|e| {
        println!("Error! {}", e);
        Vec::new()
    })
}

fn load_config_file(home_dir: &PathBuf) -> Result<Vec<String>, &str> {
    match home_dir.to_str() {
        Some(home_dir_string) => {
            match fs::File::open(home_dir_string.to_string() + "/.to/paths.cfg") {
                Ok(file) => Ok(read_paths_from_config(file)),
                Err(_) => Err("ERROR: No config file found. Create a '.to' folder in your home directory with paths.cfg inside")
            }
        },
        None => Err("Couldn't get home dir string")
    }
}

fn read_paths_from_config(file: fs::File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut paths = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(path) => paths.push(path),
            Err(err) => println!("{}", err)
        }
    }

    paths
}
