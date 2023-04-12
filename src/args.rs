use clap::Parser;

use crate::config::read_config_file;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NodemoreArgs {
    /// Path to Search
    #[clap(short, long, default_value = ".")]
    pub path: String,

    /// Time Frame
    #[clap(short, long, default_value = "1 week")]
    pub time: String,

    /// Prompt Before Deletion 
    #[arg(short = 'a', long)]
    pub prompt: bool,

    /// Whether to show file sizes (Slow)
    #[arg(short = 's', long)]
    pub show_size: bool,

    /// Verbosity Level
    #[clap(short, long, default_value = "0")]
    pub verbosity: i32
}

use crate::config::{
    Config,
    CleaningConfig
};

fn get_blank_config() -> Config {
    let blank_cleaning = CleaningConfig {
        time: String::new(),
        path: String::new()
    };
    let blank = Config {
        cleaning: blank_cleaning
    };

    blank
}

pub fn get_cleaning_time() -> String {
    let args = NodemoreArgs::parse();
    let config = read_config_file().unwrap_or(get_blank_config());
    
    if args.time != "" {
        args.time
    } else if config.cleaning.time != "" {
        config.cleaning.time
    } else {
        String::from("1 Day")
    }
}

pub fn get_cleaning_path() -> String {
    let args = NodemoreArgs::parse();
    let config = read_config_file().unwrap_or(get_blank_config());
    
    if args.path != "" {
        args.path
    } else if config.cleaning.path != "" {
        config.cleaning.path
    } else {
        String::from(".")
    }
}



