use std::env;

use anyhow::Result;
use clap::Parser;
use secrecy::ExposeSecret;

#[derive(Parser, Debug)]
#[command(name = "renshuu")]
#[command(version = env!("VERSION"))]
pub struct Args {}

pub fn get_args() -> Result<Args> {
    let args = Args::parse();
    Ok(args)
}

pub async fn run(_args: Args) -> Result<()> {
    let config = crate::config::get().expect("Failed to read configuration file");
    let client = crate::api::client::Client::new(
        "https://api.renshuu.org/v1",
        config.api_key.expose_secret(),
    )?;
    let out = client.user_profile().await.unwrap();
    println!("{}", out);
    Ok(())
}
