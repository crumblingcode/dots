use std::path::PathBuf;

use crate::App;

/// Target dotfiles
pub type Dots = Vec<PathBuf>;

/// Return targets in dotfile or panic
pub fn parse_dotfile_or_panic(cli: &App) -> Dots {
    let content =
        std::fs::read_to_string(&cli.dotfile).expect("failed to open --target-file, will exit");

    // let mut dots = Dots::default();
    let mut dots: Dots = Vec::new();
    for line in content.lines() {
        // skip comments and empty lines
        if line.starts_with("#") || line.len() == 0 {
            continue;
        }
        dots.push(PathBuf::from(line));
    }
    return dots;
}

pub fn repo_path_from_dotfile_path(fs_path: &PathBuf) -> PathBuf {
    // ensure non-absolute path
    let rel_path = match fs_path.strip_prefix("/") {
        Ok(rel) => PathBuf::from(rel),
        Err(_) => fs_path.clone(), // already relative path
    };
    // add repo prefix
    // let repo_path = // DOING
    return rel_path;
}
