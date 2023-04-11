use clap::Parser;

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

    /// Verbosity Level
    #[clap(short, long, default_value = "0")]
    pub verbosity: i32
}

