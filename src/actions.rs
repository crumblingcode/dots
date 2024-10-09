use crate::dotfile;
use log;
use std::{fs, path::PathBuf};

pub fn pull(dots: dotfile::Dots, repo_dir: &PathBuf) -> Result<(), std::io::Error> {
    for dot_path in dots.iter() {
        let repo_path = dotfile::chroot_path(&repo_dir, dot_path);
        // copy stuff
        let fs_path = expand_home(dot_path);
        log::debug!(fs_path:?, repo_path:?; "Copying dotfile");
        // TODO ensure `mkdir -p`
        let res = fs::copy(&fs_path, &repo_path);
        match res {
            Ok(_) => {}
            Err(err) => {
                log::error!(fs_path:?, repo_path:?; "Failed pulling file");
                return Err(err);
            }
        }
    }
    Ok(())
}

/// Expand HOME for paths. Will expand:
/// - `~/`
/// - `$HOME`
pub fn expand_home(path: &PathBuf) -> PathBuf {
    // TODO handle unwrap
    let home = PathBuf::from(std::env::var("HOME").unwrap());
    let path = PathBuf::from(path);

    let path = match path.strip_prefix("~/") {
        Ok(stripped) => home.join(stripped),
        Err(_) => path,
    };

    let path = match path.strip_prefix("$HOME") {
        Ok(stripped) => home.join(stripped),
        Err(_) => path,
    };
    return path;
}
