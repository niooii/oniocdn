mod upload;
mod config;

use std::{path::{Path, PathBuf}, str::FromStr};
use reqwest::Url;
use toml::toml;
use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use config::Config;
use tokio::fs::File;

pub const CLOUD_URL: &str = "cloud_url";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload files to the cloud.
    Upload { path: String },
    /// Set the cloud's base url.
    SetUrl { url: String }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = dirs::config_dir().unwrap_or_default()
        .join("config.toml");

    let mut config = Config::from_file(&config_path).await?;

    match &cli.command {
        
        Commands::Upload { path } => {
            if let Err(e) = Url::parse(&config.cloud_url) {
                eprintln!("Use the set-url command to set a cloud url.");
                return Err(Error::from(e));
            }
            if let Err(e) = upload::upload_file(&Path::new(path), &config).await {
                eprintln!("Failed to upload file: {e}");
            }
            println!("done");
        },

        Commands::SetUrl { url } => {
            if let Err(e) = Url::parse(url) {
                eprintln!("Invalid cloud url: \"{}\". Check the input and try again.", url);
                return Err(Error::from(e));
            }

            config.cloud_url = url.clone();
            
            if let Err(e) = config.save_to(&config_path).await {
                eprintln!("Failed to save config changes: {e}");
                return Err(e);
            }

            println!("Done.");
        }

    }
    
    Ok(())
}
