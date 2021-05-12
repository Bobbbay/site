#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod server;
mod tools;

use anyhow::Result;

fn main() -> Result<()> {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("../misc/cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref _matches) = matches.subcommand_matches("run") {
        server::run()?;
    } else if let Some(ref _matches) = matches.subcommand_matches("build_posts") {
        tools::build_posts()?;
    }

    Ok(())
}
