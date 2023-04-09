mod time;
mod config;

use time::{
    get_unix_last_modified,
    human_to_unix_time
};
use config::read_config_file; 

fn main() {
    /*
    match get_last_modified("farm/index.js") {
        Ok(time) => println!("{time:?}"),
        Err(error) => eprintln!("{}", error),
    }
    */

    // let config = read_config_file().unwrap();
    // println!("{}", config.cleaning.time)
    
    human_to_unix_time("1 month");
}
