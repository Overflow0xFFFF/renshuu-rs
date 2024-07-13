use std::env;

use anyhow::Result;
use clap::Parser;
use secrecy::ExposeSecret;

use crate::Renshuu;

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
    let client = Renshuu::new(config.api_key.expose_secret())?;
    println!("{}", client.user().get_profile().await.unwrap());
    println!("{}", client.user().get_lists().await.unwrap());
    println!(
        "{}",
        client
            .user()
            .get_list_of_all_terms_by("kanji")
            .await
            .unwrap()
    );
    Ok(())
}
