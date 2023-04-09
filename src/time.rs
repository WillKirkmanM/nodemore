use std::fs;
use std::time::{
    SystemTime,
    Duration, 
    UNIX_EPOCH
};

use humantime::parse_duration;

pub fn get_unix_last_modified(path: &str) -> std::io::Result<Duration> {
    let metadata = fs::metadata(path)?;

    if let Ok(time) = metadata.modified() {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        Ok(duration)
    } else {
        let error = std::io::Error::new(std::io::ErrorKind::Other, format!("Cannot get Modified Date of {} (Not Supported)", path));
        Err(error)
    }
}

pub fn human_to_unix_time(human_time: &str) { 
    let a = parse_duration(human_time).unwrap();
    println!("{:?}", a);

    let now = SystemTime::now();

    let d = Duration::new(a, 0);
}
