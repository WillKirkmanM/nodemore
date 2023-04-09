use std::fs;
use std::path::Path;

use crate::time::{
    human_to_unix_time,
    get_unix_last_modified
};

pub fn should_clean_dir(dir: &str) -> bool {

    let node_modules_exists = Path::new(&(dir.to_owned() + "/" + "node_modules")).exists();

    if node_modules_exists {
        let dir = fs::read_dir(dir).unwrap();
        for file in dir {
            let file_path = file.unwrap().path();
            let file_path_str = file_path.to_str().unwrap();

            let should = should_clean(file_path_str).unwrap();
            if should == false {
                return false
            }
        }
        true
    } else {
        false
    }
}

pub fn should_clean(path: &str) -> Result<bool, std::io::Error> {
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
