mod time;
mod config;
mod dir;
mod args;

use dir::check_dir;
use args::NodemoreArgs;
use clap::Parser;

fn main() {
    check_dir("../../Projects")

    /*
    if should_clean_dir("Projects") {
        println!("damn")
    } else {
        println!("cool u good")
    };
    */
}
