use std::path::Path;
use toml::toml;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub cloud_url: String
}

impl Config {
    /// Creates a default configuration file at the specified path.
    pub async fn create_at_path(path: &Path) -> Result<Self> {
        let mut file = File::create(path).await?;

        let toml = 
        toml! {
            cloud_url = ""            
        };

        let toml_str = toml::to_string(&toml)?;

        file.write_all(toml_str.as_bytes()).await?;

        Ok(
            toml::from_str(&toml_str)?
        )
     }

    pub async fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        drop(file);
        
        Ok(
            match toml::from_str::<Config>(&contents) {
                Err(e) => Self::create_at_path(path).await?,
                Ok(t) => t
            }
        )
    }

    pub async fn save_to(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string(self)?;

        let mut file = File::create(path).await?;

        file.write_all(contents.as_bytes()).await?;

        Ok(())
    }
}