mod time;
mod config;
mod dir;

use crate::dir::should_clean_dir;

fn main() {
    if should_clean_dir("Projects") {
        println!("damn")
    } else {
        println!("cool u good")
    };
}
