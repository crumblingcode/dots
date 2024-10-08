mod cli;
mod dotfile;

use clap::Parser;
use cli::Args;

fn main() {
    println!("Hello, world!");

    // cli
    let cli = Args::parse();
    println!("{:#?}", cli);

    // work
    let tgt_files = dotfile::parse_dotfile(&cli);
    println!("{:#?}", tgt_files);
}
