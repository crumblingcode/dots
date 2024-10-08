use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Toggle No-op (dry-run)
    ///
    /// Only print actions, without performing them on file system
    #[arg(short, long)]
    pub noop: bool,

    /// Path to file containing targeted/managed file (paths)
    ///
    /// The file should contain on path per row, and comments (#) and empty
    /// rows will be ignored
    #[arg(short, long, default_value_t = String::from("dotfiles.txt"))]
    pub dotfile: String,
    // sub1
}
