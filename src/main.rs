mod time;
mod config;
mod dir;

use dir::should_clean_dir;

fn main() {
    dbg!(should_clean_dir("farm"));
}
