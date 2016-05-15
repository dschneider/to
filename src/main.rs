use std::env;
use std::process;
use std::fs;
use std::io::prelude::*;

mod config;
mod input;

macro_rules! write_to_terminal_through_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    match read_folder_argument() {
        Some(desired_folder) => look_for_folder(&desired_folder),
        None => println!("Error: Missing folder name. Usage: to [foldername]\n")
    };
}


fn read_folder_argument() -> Option<String> {
    env::args().nth(1)
}

fn look_for_folder(folder_name: &str) {
    let paths: Vec<String> = config::get_paths_from_config_in_home_folder();
    let matches: Vec<String> = search_matching_folders_in_paths_from_config(paths, folder_name);

    if matches.len() > 1 {
        prompt_user_for_input(&matches);
    } else if matches.len() == 1 {
        write_to_terminal_through_stderr!("One matching folder found");
        println!("{}", matches[0]);
    } else {
        write_to_terminal_through_stderr!("No matching folders found");
        process::exit(0);
    }
}

fn search_matching_folders_in_paths_from_config(paths: Vec<String>, desired_folder: &str) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for path in &paths {
        read_entries_in_path(&path, desired_folder, &mut matches);
    }

    matches
}

fn read_entries_in_path(path: &str, desired_folder: &str, vec: &mut Vec<String>) {
    match fs::read_dir(path) {
        Ok(files) => {
            for file in files {
                 match_folder_names(path, file, desired_folder, vec);
            }
        },
        Err(err) => println!("{}", err)
    }
}

fn match_folder_names(path: &str, file: Result<std::fs::DirEntry, std::io::Error>, desired_folder: &str, vec: &mut Vec<String>) {
    match file {
        Ok(file_object) => {
            if file_object.path().is_dir() {
                match file_object.file_name().to_str() {
                    Some(folder_string) => {
                        if folder_string.contains(desired_folder) {
                            vec.push(path.to_string() + folder_string);
                        }
                    },
                    None => panic!("No valid unicode")
                }
            }
        },
        Err(err) => panic!("{}", err)
    }
}

fn prompt_user_for_input(matches: &Vec<String>) {
    let mut chosen: bool = false;

    while !chosen {
        show_matching_folders(matches);

        match input::read_user_input() {
            Ok(choice) => {
                if choice > matches.len() as i32 - 1 {
                    chosen = false;
                    write_to_terminal_through_stderr!("Please enter one of the shown inputs");
                } else {
                    chosen = true;
                    println!("{}", matches[choice as usize]);
                }
            },
            Err(_) => {
                chosen = false;
                println!("{}", "Please enter one of the shown numbers");
            }
        }
    }
}

fn show_matching_folders(matches: &Vec<String>) {
    write_to_terminal_through_stderr!("Multiple matching folders found!");
    write_to_terminal_through_stderr!("Please choose a folder:\n");

    let mut index = 0;

    for mat in matches {
        write_to_terminal_through_stderr!("{}: {}", index, mat);
        index = index + 1;
    }

    write_to_terminal_through_stderr!("");
}
