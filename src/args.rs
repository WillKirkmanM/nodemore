use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NodemoreArgs {
    /// Prompt Before Deletion 
    #[clap(short, long)]
    pub prompt: bool,

    /// Verbosity Level
    #[clap(short, long, default_value = "0")]
    pub verbosity: i32

}


