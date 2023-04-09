use std::fs;
use std::path::Path;

use crate::time::{
    human_to_unix_time,
    get_unix_last_modified
};

pub fn check_dir(dir: &str) {
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
                should_clean_dir_no_checks(file_path.clone().to_str().unwrap());
                continue;
            } else {
                println!("File {:?}", &file_path);
                let file_path_str = file_path.to_str().unwrap();

                let should = should_clean_file(file_path_str).unwrap();
                if should == false {
                    // return false
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
                should_clean_dir(file_path.clone().to_str().unwrap());
                continue;
            } else {
                println!("File {:?}", &file_path);
                let file_path_str = file_path.to_str().unwrap();

                let should = should_clean_file(file_path_str).unwrap();
                if should == false {
                    // return false
                }
            };
        }
        true
}

pub fn should_clean_file(path: &str) -> Result<bool, std::io::Error> {
    use super::config::read_config_file;

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
