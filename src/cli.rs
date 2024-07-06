use std::env;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "renshuu")]
#[command(version = env!("VERSION"))]
pub struct Args {}

pub fn get_args() -> Result<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(_args: Args) -> Result<()> {
    let _config = crate::config::get().expect("Failed to read configuration file");
    Ok(())
}
