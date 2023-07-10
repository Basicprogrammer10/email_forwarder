#![feature(type_name_of_val)]

use anyhow::Result;

mod app;
mod config;

fn main() -> Result<()> {
    let mut app = app::App::new("config.toml")?;
    app.check_emails()?;

    Ok(())
}
