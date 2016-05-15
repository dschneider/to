use std::io;
use std::env;
use std::fs;
use std::process;
use std::io::prelude::*;
use std::io::BufReader;

macro_rules! write_to_terminal(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn search_matching_folders_in_paths(paths: Vec<String>, desired_folder: String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for path in &paths {
        read_entries_in_path(path, &desired_folder, &mut vec);
    }

    vec
}

fn match_folder_names(path: &String, file: Result<std::fs::DirEntry, std::io::Error>, desired_folder: &String, vec: &mut Vec<String>) {
    match file {
        Ok(file_object) => {
            if file_object.path().is_dir() {
                match file_object.file_name().into_string() {
                    Ok(string) => {
                        if string.contains(&desired_folder.clone()) {
                            vec.push(path.to_owned() + &string.to_owned());
                        }
                    },
                    Err(_) => println!("zolo")
                }
            }
        },
        Err(err) => println!("{}", err)
    }
}

fn read_entries_in_path(path: &String, desired_folder: &String, vec: &mut Vec<String>) {
    match fs::read_dir(path) {
        Ok(files) => {
            for file in files {
                 match_folder_names(path, file, desired_folder, vec);
            }
        },
        Err(err) => println!("{}", err)
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

fn load_config_file(home_dir: &std::path::PathBuf) -> Result<Vec<String>, &'static str> {
    match fs::File::open(home_dir.to_str().unwrap().to_string() + "/.to/paths.cfg") {
        Ok(file) => Ok(read_paths_from_config(file)),
        Err(_) => Err("ERROR: No config file found. Create a 'to' folder in your home directory with paths.cfg inside")
    }
}

fn get_paths_from_config_in_home_folder() -> Vec<String> {
    let loaded_paths: Vec<String> = match env::home_dir() {
        Some(ref home_dir) => match load_config_file(home_dir) {
            Ok(paths) => paths,
            Err(err) => {
                println!("{}", err);
                Vec::new()
            }
        },
        None => panic!("No home directory found!")
    };

    loaded_paths
}

fn look_for_folder(folder_name: String) {
    let paths: Vec<String> = get_paths_from_config_in_home_folder();
    let matches: Vec<String> = search_matching_folders_in_paths(paths, folder_name);

    if matches.len() > 1 {
        prompt_user_for_input(&matches);
    } else if matches.len() == 1 {
        write_to_terminal!("One matching folder found");
        println!("{}", matches[0]);
    } else {
        write_to_terminal!("No matching folders found");
        process::exit(0);
    }
}

fn prompt_user_for_input(matches: &Vec<String>) {
    let mut chosen: bool = false;

    while !chosen {
        write_to_terminal!("Multiple matching folders found!");

        let mut input = String::new();
        write_to_terminal!("Please choose a folder:\n");

        let mut index = 0;
        for mat in matches {
            write_to_terminal!("{}: {}", index, mat);
            index = index + 1;
        }

        write_to_terminal!("");

        io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

        let input: String = input.replace("\n", "");
        let choice: i32 = input.parse().ok().expect("Wanted a number");

        if choice > matches.len() as i32 - 1 {
            chosen = false;
            write_to_terminal!("Please enter one of the shown inputs");
        } else {
            chosen = true;
            println!("{}", matches[choice as usize]);
        }
    }
}

fn read_folder_argument() -> Option<String> {
    env::args().nth(1)
}

fn main() {
    match read_folder_argument() {
        Some (folder_name) => look_for_folder(folder_name),
        None => println!("Error: Missing folder name. Usage: to [foldername]\n")
    };
}
