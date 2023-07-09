use std::fs;

use anyhow::Result;

mod config;

fn main() -> Result<()> {
    let raw_config = fs::read_to_string("config.toml")?;
    let config = toml::from_str::<config::Config>(&raw_config)?;

    dbg!(config);

    Ok(())
}
