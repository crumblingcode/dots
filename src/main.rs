mod actions;
mod cli;
mod dotfile;

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
        cli::SubCmd::Pull {} => actions::pull(tgt_files, &app.get_dotfiles_dir_path())
            .expect("Failed pulling - Exiting"),
        cli::SubCmd::Push {} => {
            println!("TODO!")
        }
        cli::SubCmd::Diff {} => {
            println!("TODO!")
        }
    }
}
