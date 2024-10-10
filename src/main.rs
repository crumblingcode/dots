mod actions;
mod cli;
mod dotfile;

use std::fs;

use clap::Parser;
use cli::App;

fn init_logger() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    // structured_logger::Builder::with_level("info").init();
}
fn main() {
    // init logging
    init_logger();

    // cli
    let app = App::parse();
    log::debug!(app:?; "Running with live config");

    // parse dotfile
    let tgt_files = dotfile::parse_dotfile_or_panic(&app);
    log::debug!(tgt_files:?; "Found files to target");

    // do stuff
    match app.command {
        cli::SubCmd::Pull {} => {
            let res = actions::pull(tgt_files, &app.get_dotfiles_dir_path());
            match res {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        cli::SubCmd::Push {} => {
            println!("TODO!");
            fs::copy("/.config/nvim/init.lua", "./tmp.txt").unwrap();
        }
        cli::SubCmd::Diff {} => {
            println!("TODO!");
        }
    }
}
