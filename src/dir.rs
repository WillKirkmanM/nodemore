use std::fs;
use std::io::stdin;
use std::path::Path;
use std::process::exit;

use crate::args::{
    NodemoreArgs,
    get_cleaning_time,
    get_cleaning_path
};
use crate::time::{get_unix_last_modified, human_to_unix_time};

use clap::Parser;
use colored::Colorize;
use human_bytes::human_bytes;

pub fn init() {
    let cleaning_time = get_cleaning_time();
    let cleaning_path = get_cleaning_path();

    let list = projects_to_clean(&cleaning_path);
    if list.len() == 0 {
        println!("There are {} Projects Not Accessed in the last {} Good Job! 😁", "0".bright_green(), cleaning_time.bright_green());
        exit(0)
    }
        
    println!("NodeJS Projects Not Accessed in the last {}:", cleaning_time.bright_green());
    ask_to_clean(list);
}

trait FormatSize: std::fmt::Display {
    fn format_size(&self) -> colored::ColoredString;
}

impl FormatSize for u64 {
    fn format_size(&self) -> colored::ColoredString {
        let bytes = *self as u64;
        let hundred_megabytes = 100_000_000;
        let half_a_gigabyte = 500_000_000;

        if bytes < hundred_megabytes {
            let readable = human_bytes(bytes as f64);
            String::from(readable).yellow()
        } else if bytes > hundred_megabytes && bytes < half_a_gigabyte {
            let readable = human_bytes(bytes as f64);
            String::from(readable).red()
        } else if bytes >= half_a_gigabyte {
            let readable = human_bytes(bytes as f64);
            String::from(readable).bright_red()
        } else {
            let readable = human_bytes(bytes as f64);
            String::from(readable).yellow()
        }
    }
}

pub fn contains_only_folders(dir: &str) -> bool {
    let contents;
    match fs::read_dir(dir) {
        Ok(content) => contents = content,
        Err(_) => {
            println!("Hey! It looks like you're in a folder that we can't access❕Try going into a different folder.");
            exit(1)
        }
    };

    for file in contents {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            return false;
        }
    }
    true
}

pub fn get_directory_size(dir: &str) -> u64 {
    let mut total_size: u64 = 0;
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            total_size += get_directory_size(&entry_path.to_str().unwrap());
        } else {
            total_size += fs::metadata(entry_path).unwrap().len();
        }
    }
    total_size
}

pub fn ask_to_clean(list_vec: Vec<String>) {
    let args = NodemoreArgs::parse();

    if args.prompt {
        for project in list_vec.iter() {
            let mut project_name = "";
            if let Some(index) = project.rfind('/') {
                project_name = &project[index + 1..];
            }

            let mut message = 
                format!("[{}]: {}",
                    "-".red(),
                    project_name.bright_green(),
                );

            if args.show_size {
                let size = get_directory_size(project).format_size();
                message = message + " " + &format!("(~{})", size).to_string()
            }
            if args.verbosity >= 1 {
                message = message + " " + &format!("({})", project.bright_green()).to_string()
            }

            println!("{}", message)
        }
    } 
    println!(
        "Do you want to {} these ({}) Project(s)? (Y/n)",
        "clean".red(),
        list_vec.len().to_string().bright_green(),
    );

    let mut answer = String::new();
    stdin().read_line(&mut answer).unwrap();
    answer = answer.trim().to_string();

    if answer == "Y" || answer == "y" {
        for (mut value, project) in list_vec.iter().enumerate() {
            value = value + 1;
            delete_node_modules(project, value as u32);
        }
        println!(
            "Done! Cleaned ({}) Project(s) from your Hard Disk!",
            list_vec.len().to_string().bright_green(),
        )
    } else {
        println!("Bye! 👋")
    }
}

pub fn delete_node_modules(dir: &str, value: u32) {
    let node_modules_path = dir.to_string() + "/node_modules/";
    let args = NodemoreArgs::parse();

    let mut project_name = "";
    if let Some(index) = dir.rfind('/') {
        project_name = &dir[index + 1..];
    }

    match fs::remove_dir_all(&node_modules_path) {
        Ok(_) => {
            let mut message = 
                format!("[{}]: ({}) {} {}!",
                        "-".red(),
                        value.to_string().bright_green(),
                        "Cleaned".bright_green(),
                        project_name.bright_green(),
                    );

            if args.show_size {
                let dir_size = get_directory_size(dir).format_size().bright_green();
                message = message + " " + &format!("(Now ~{})", dir_size).to_string()
            }

            if args.verbosity >= 1  {
                message = message + " " + &format!("({})", dir.bright_green()).to_string()
            }
            
            println!("{}", message)
        }
        Err(err) => {
            let mut message = 
                format!("There was an {} {}{}",
                        "error deleting".red(),
                        project_name.bright_green(),
                        "/node_modules/".bright_green(),
                    );

            if args.verbosity >= 1 {
                message = message + " " + &format!("({})", dir.bright_green()).to_owned()
            }

            message = message + &format!("\n{}", err).to_string();

            println!("{}", message)
        }
    }
}

pub fn projects_to_clean(dir: &str) -> Vec<String> {
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
            dir_vec.append(&mut projects_to_clean(dir_path_str))
        }
        if is_dir {
            let package_json_exists =
                Path::new(&(dir_path.to_str().unwrap().to_string() + "/" + "package.json"))
                    .try_exists()
                    .unwrap();
            if package_json_exists {
                let should = should_clean_dir_with_checks(dir_path_str);
                let args = NodemoreArgs::parse();

                if should {
                    if args.prompt {
                        let mut message = 
                            format!("would you like to {} {}? (Y/n))",
                                    "clean".red(),
                                    dir_name.bright_green()
                            );

                        if args.verbosity >= 1 {
                            message = message + " " + &format!(" {}", dir_path_str.bright_green() ).to_string()
                        }

                        println!("{}", message);

                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        input = input.trim().to_string();

                        if input == "Y" || input == "y" {
                            dir_vec.push(dir_path_str.to_string());
                        } else {
                            continue;
                        }
                    } else {
                        dir_vec.push(dir_path_str.to_string());

                        let mut message = 
                            format!("[{}]: {}",
                                "-".red(),
                                dir_name.bright_green(),
                            );

                        if args.show_size {
                            let size = get_directory_size(dir_path.to_str().unwrap()).format_size();
                            message = message + " " + &format!("(~{})", size).to_string()
                        }
                        if args.verbosity >= 1 {
                            message = message + " " + &format!("({})", dir_path_str.bright_green()).to_string()
                        }

                        println!("{}", message)
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
        should_clean_dir(dir)
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
            let file_path_str = file_path.to_str().unwrap();

            let should = should_clean_file(file_path_str).unwrap();
            if should == false {
                return false;
            }
        };
    }
    true
}

pub fn should_clean_file(path_to_file: &str) -> Result<bool, std::io::Error> {
    let cleaning_time = get_cleaning_time();

    match get_unix_last_modified(path_to_file) {
        Ok(time) => {
            if time > human_to_unix_time(cleaning_time) {
                Ok(false)
            } else {
                Ok(true)
            }
        }
        Err(error) => Err(error),
    }
}
