use crate::dotfile;
use log;

pub fn pull(dots: dotfile::Dots) {
    println!("--- Pulling!");
    for dot in dots.iter() {
        let rel_dot = dotfile::repo_path_from_dotfile_path(dot);
        // copy stuff
        // fs::copy(dot, to)
        log::info!(rel_dot:?; "found relative path");
    }
}
