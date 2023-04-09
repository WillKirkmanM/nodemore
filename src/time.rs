use std::fs;
use std::time::{
    SystemTime,
    UNIX_EPOCH
};

use humantime::parse_duration;

pub fn get_unix_last_modified(path: &str) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(path)?;

    if let Ok(time) = metadata.modified() {
        let duration = time.duration_since(UNIX_EPOCH).unwrap().as_secs() ;
        Ok(duration)
    } else {
        let error = std::io::Error::new(std::io::ErrorKind::Other, format!("Cannot get Modified Date of {} (Not Supported)", path));
        Err(error)
    }
}

pub fn human_to_unix_time(mut human_time: String) -> u64 { 
    human_time = human_time.to_lowercase();

    let duration = parse_duration(human_time.as_str()).unwrap().as_secs();
    let unix_current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let human_as_unix_time = unix_current_time - duration;
    human_as_unix_time
}

