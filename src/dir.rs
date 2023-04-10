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

pub fn amount_to_clean() -> u32 {
}

pub fn check_dir(dir: &str) {
    let dir_list = fs::read_dir(dir).unwrap();

    for directory in dir_list {
        let directory = directory.unwrap();
        let dir_path = &directory.path();
        let dir_path_str = dir_path.to_str().unwrap();
        let tmp = &directory.file_name();
        let dir_name = tmp.to_str().unwrap();

        let is_dir = fs::metadata(dir_path).unwrap().is_dir();
        if is_dir && contains_only_folders(dir_path_str) {
            check_dir(dir_path_str)
        }
        if is_dir {
            if Path::new(&(dir_path.to_str().unwrap().to_string() + "/" + "package.json")).try_exists().unwrap() {
                let should = should_clean_dir_no_checks(dir_path_str);
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
                        if args.verbosity >= 1 {
                            println!("[{}]: {} ({})", "-".red(), dir_name.bright_green(), dir_path_str.bright_green())
                        } else {
                            println!("[{}]: {}", "-".red(), dir_name.bright_green())
                        }
                    }
                }
            } 
        }
    }
} 


pub fn should_clean_dir(dir: &str) -> bool {
    let node_modules_exists = Path::new(&(dir.to_owned() + "/" + "node_modules")).exists();

    if node_modules_exists {
        let dir_list = fs::read_dir(dir).unwrap();
        for file in dir_list {
            let file_name = file.as_ref().unwrap().file_name();
            let file_path = file.unwrap().path();

            let is_dir = fs::metadata(&file_path).unwrap().is_dir();
            if is_dir && file_name == "node_modules" {
                continue;
            } else if is_dir {
                let should = should_clean_dir(file_path.clone().to_str().unwrap());
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
    } else {
        false
    }
}

pub fn should_clean_dir_no_checks(dir: &str) -> bool {
        let dir_list = fs::read_dir(dir).unwrap();
        for file in dir_list {
            let file_name = file.as_ref().unwrap().file_name();
            let file_path = file.unwrap().path();

            let is_dir = fs::metadata(&file_path).unwrap().is_dir();
            if is_dir {
                let should = should_clean_dir(file_path.clone().to_str().unwrap());
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
