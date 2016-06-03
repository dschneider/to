use std::env;
use std::process;
use std::fs;
use std::io::Write;

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
    let mut matches = vec!();

    for path in &paths {
        read_entries_in_path(&path, desired_folder, &mut matches);
    }

    matches
}

fn read_entries_in_path(path: &str, desired_folder: &str, matches: &mut Vec<String>) {
    match fs::read_dir(path) {
        Ok(files) => {
            for file in files {
                 if let Some(folder_name) = match_folder_names(file, desired_folder) {
                     matches.push(path.to_owned() + &folder_name);
                 }
            }
        }
        Err(err) => println!("{}", err)
    }
}

fn match_folder_names(file: Result<std::fs::DirEntry, std::io::Error>, desired_folder: &str) -> Option<String> {
    match file {
        Ok(ref dir_entry) if dir_entry.path().is_dir() => {
            let file_name = dir_entry.file_name();
            let folder_name = file_name.to_str().expect("File name is not valid unicode");
            if folder_name.contains(desired_folder) {
                Some(folder_name.to_owned())
            } else {
                None
            }
        }
        Ok(_) => None,
        Err(err) => panic!("{}", err)
    }
}

fn prompt_user_for_input(matches: &Vec<String>) {
    loop {
        show_matching_folders(matches);

        match input::read_user_input() {
            Ok(choice) if choice > matches.len() as u16 - 1 => {
                write_to_terminal_through_stderr!("Please enter one of the shown inputs");
            }
            Ok(choice) => {
                println!("{}", matches[choice as usize]);
                break;
            }
            Err(_) => {
                println!("{}", "Please enter one of the shown numbers");
            }
        }
    }
}

fn show_matching_folders(matches: &Vec<String>) {
    write_to_terminal_through_stderr!("Multiple matching folders found!");
    write_to_terminal_through_stderr!("Please choose a folder:\n");

    for (index, mat) in matches.iter().enumerate() {
        write_to_terminal_through_stderr!("{}: {}", index, mat);
    }

    write_to_terminal_through_stderr!("");
}
