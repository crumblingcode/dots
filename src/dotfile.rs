use std::path::PathBuf;

use crate::Args;

/// Target dotfiles
#[derive(Debug)]
pub struct Dots(Vec<PathBuf>);

/// Return targets in dotfile or panic
pub fn parse_dotfile_or_panic(cli: &Args) -> Dots {
    let content =
        std::fs::read_to_string(&cli.dotfile).expect("failed to open --target-file, will exit");

    // let mut dots = Dots::default();
    let mut dots = Vec::new();
    for line in content.lines() {
        // skip comments and empty lines
        if line.starts_with("#") || line.len() == 0 {
            continue;
        }
        dots.push(PathBuf::from(line));
    }
    return Dots(dots);
}
