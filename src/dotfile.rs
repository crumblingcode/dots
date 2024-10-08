use crate::Args;

#[derive(Debug)]
pub struct Targets {}

pub fn parse_dotfile(cli: &Args) -> Targets {
    println!("hello from target_file.rs");

    let content =
        std::fs::read_to_string(&cli.dotfile).expect("failed to open --target-file, will exit");

    for line in content.lines() {
        println!("{:?}", line)
    }
    return Targets {};
}
