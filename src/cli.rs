use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct App {
    /// Toggle No-op (dry-run)
    ///
    /// Only print actions, without performing them on file system
    #[arg(short, long, global = true)]
    pub noop: bool,

    /// Path to file containing targeted/managed file (paths)
    ///
    /// The file should contain on path per row, and comments (#) and empty
    /// rows will be ignored
    #[arg(short, long, global = true, default_value = "dotfiles.txt")]
    pub dotfile: PathBuf,

    /// Path to repo dir, to where dotfiles will be pulled
    ///
    #[arg(long, global = true)]
    pub dotfiles_dir: Option<PathBuf>,

    /// Subcommands
    #[clap(subcommand)]
    pub command: SubCmd,
}

/// Command to run
#[derive(Debug, Subcommand)]
pub enum SubCmd {
    /// Pull files from filesystem into current dir
    Pull {},
    /// TODO
    Push {},
    /// TODO
    Diff {},
}

impl App {
    pub fn get_dotfiles_dir_path(&self) -> PathBuf {
        match &self.dotfiles_dir {
            Some(dotfiles_dir) => {
                // use dir
                return PathBuf::from(dotfiles_dir);
            }
            None => {
                // infer from dotfile
                let mut p = self.dotfile.clone();
                p.pop();
                return p;
            }
        }
    }
}
