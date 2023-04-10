mod time;
mod config;
mod dir;
mod args;

use dir::check_dir;
use args::NodemoreArgs;
use clap::Parser;

use config::read_config_file;
use colored::Colorize;

fn main() {

    let config = read_config_file().unwrap();
    println!("NodeJS Projects Not Accessed in the last {}:", (config.cleaning.time).bright_green());
    let count = check_dir("../../Projects");
    println!("Do you want to clean these {} Projects?", count.to_string().green())

    /*
    if should_clean_dir("Projects") {
        println!("damn")
    } else {
        println!("cool u good")
    };
    */
}
