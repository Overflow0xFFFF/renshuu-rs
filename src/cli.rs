use std::env;

use anyhow::Result;
use clap::Parser;
use secrecy::ExposeSecret;
use serde_json;

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
    println!(
        "{}",
        serde_json::to_string(&client.user().get_profile().await.unwrap()).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&client.user().get_lists().await.unwrap()).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&client.user().get_list_by("5005186").await.unwrap()).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(
            &client
                .user()
                .get_list_of_all_terms_by("kanji")
                .await
                .unwrap()
        )
        .unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&client.reibun().search("learn Japanese").await.unwrap()).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&client.reibun().search_by("1000").await.unwrap()).unwrap()
    );
    Ok(())
}
