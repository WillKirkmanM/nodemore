use std::fs;
use std::path::Path;
use std::io::stdin;

use crate::time::{
    human_to_unix_time,
    get_unix_last_modified
};
use crate::args::NodemoreArgs;
use crate::config::read_config_file;

use colored::Colorize;
use clap::Parser;

pub fn init() {
    let config = read_config_file().unwrap();
    println!("NodeJS Projects Not Accessed in the last {}:", (config.cleaning.time).bright_green());
    let list = check_dir("../../Projects");
    ask_to_clean(list)
}

pub fn contains_only_folders(dir: &str) -> bool {
    let contents = fs::read_dir(dir).unwrap();
    for file in contents {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            return false;
        }
    }
    true
}

pub fn ask_to_clean(list_vec: Vec<String>) {
    println!("Do you want to {} these ({}) Projects? (Y/n)", "clean".red(), list_vec.len().to_string().bright_green());

    let mut answer = String::new();
    stdin().read_line(&mut answer).unwrap();
    answer = answer.trim().to_string();

    if answer == "Y" || answer == "y" {
        for (mut value, project) in list_vec.iter().enumerate() {
            value = value + 1;
            delete_node_modules(project, value as u32);
        }
    } else {
        println!("Bye! 👋")
    }
}

pub fn delete_node_modules(dir: &str, value: u32) {
    let node_modules_path = dir.to_string() + "/node_modules/";
    let args = NodemoreArgs::parse();

    if let Some(index) = dir.rfind('/') {
        let project_name = &dir[index + 1..];

        match fs::remove_dir_all(&node_modules_path) {
            Ok(_) => {
                if args.verbosity >= 1 {
                    println!("[{}]: [{}] {}! {} ({})", "Cleaned".bright_green(), "-".red(), value.to_string().bright_green(), project_name.bright_green(), dir.bright_green())
                } else {
                    println!("[{}]: {}! {}", "Cleaned".bright_green(), "-".red(), project_name.bright_green())
                }
            },
            Err(err) => {
                if args.verbosity >= 1 {
                    eprintln!("There was an {} {}{} ({})\n{}", "error deleting".red(), project_name.bright_green(), "/node_modules/".bright_green(), node_modules_path.bright_green(), err)
                } else {
                    eprintln!("There was an {} {}{}\n{}", "error deleting".red(), project_name.bright_green(), "/node_modules/".bright_green(), err)
                }
            }
        }   
    }
    
}

pub fn check_dir(dir: &str) -> Vec<String> {
    let mut dir_vec: Vec<String> = vec![];
    let dir_list = fs::read_dir(dir).unwrap();

    for directory in dir_list {
        let directory = directory.unwrap();
        let dir_path = &directory.path();
        let dir_path_str = dir_path.to_str().unwrap();
        let tmp = &directory.file_name();
        let dir_name = tmp.to_str().unwrap();

        let is_dir = fs::metadata(dir_path).unwrap().is_dir();
        if is_dir && contains_only_folders(dir_path_str) {
            dir_vec.append(&mut check_dir(dir_path_str))
        }
        if is_dir {
            let package_json_exists = Path::new(&(dir_path.to_str().unwrap().to_string() + "/" + "package.json")).try_exists().unwrap();
            if package_json_exists {
                let should = should_clean_dir(dir_path_str);
                let args = NodemoreArgs::parse();
                if should {
                    if args.prompt {
                        if args.verbosity >= 1 {
                            println!("Would you like to {} {}? ({})", "clean".red(), dir_name.bright_green(), dir_path_str.bright_green());
                        } else {
                            println!("Would you like to {} {}?", "clean".red(), dir_name.bright_green());
                        }

                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        input = input.trim().to_string();

                        if input == "Y" || input == "y" {
                            todo!("Delete Node Modules")
                        } else {
                            continue
                        }
                    } else {
                        dir_vec.push(dir_path_str.to_string());
                        if args.verbosity >= 1 {
                            println!("[{}]: {} ({})", "-".red(), dir_name.bright_green(), dir_path_str.bright_green());
                        } else {
                            println!("[{}]: {}", "-".red(), dir_name.bright_green());
                        }
                    }
                }
            } 
        }
    }
    dir_vec
} 


pub fn should_clean_dir_with_checks(dir: &str) -> bool {
    let node_modules_exists = Path::new(&(dir.to_owned() + "/" + "node_modules")).exists();

    if node_modules_exists {
        should_clean_dir(dir) // <- Bool
    } else {
        false
    }
}

pub fn should_clean_dir(dir: &str) -> bool {
        let dir_list = fs::read_dir(dir).unwrap();
        for file in dir_list {
            let file_name = file.as_ref().unwrap().file_name();
            let file_path = file.unwrap().path();

            let is_dir = fs::metadata(&file_path).unwrap().is_dir();
            if is_dir {
                let should = should_clean_dir_with_checks(file_path.clone().to_str().unwrap());
                if should {
                    println!("{:?} Should be deleted", file_name)
                }
                continue;
            } else {
                // println!("File {:?}", &file_path);
                let file_path_str = file_path.to_str().unwrap();

                let should = should_clean_file(file_path_str).unwrap();
                if should == false {
                    return false
                }
            };
        }
        true
}

pub fn should_clean_file(path: &str) -> Result<bool, std::io::Error> {
    let config = read_config_file().unwrap();
    match get_unix_last_modified(path){
        Ok(time) => {
            if time > human_to_unix_time(config.cleaning.time) {
                Ok(false)
            } else {
                Ok(true)
            }
        },
        Err(error) => {
            Err(error)
        }

    }
}
