use anyhow::Error;
use dirs::home_dir;

use crate::core::app::AppResult;

pub fn get_home_dir() -> AppResult<String> {
    match home_dir() {
        Some(home_dir) => Ok(format!("{}", home_dir.display())),
        None => Err(Error::msg("Unable to find home directory.")),
    }
}
