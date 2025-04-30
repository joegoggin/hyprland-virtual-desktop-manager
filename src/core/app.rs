use super::config::Config;

pub type AppResult<T> = anyhow::Result<T>;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> AppResult<()> {
        let mut config = Config::default();
        let result = config.load_from_file();

        if let Err(_) = result {
            config.prompt_user().await?;
        }

        Ok(())
    }
}
