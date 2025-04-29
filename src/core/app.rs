use clap::Parser;

use super::clap::Args;

pub type AppResult<T> = anyhow::Result<T>;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> AppResult<()> {
        let args = Args::parse();

        println!("{:?}", args);

        Ok(())
    }
}
