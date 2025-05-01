use anyhow::Error;
use serde_json::Value;

use crate::utils::terminal_command::TerminalCommand;

use super::app::AppResult;

pub struct Hyprctl;

impl Hyprctl {
    pub fn monitors() -> AppResult<Vec<Value>> {
        let output = TerminalCommand::new("hyprctl monitors -j").run_with_output()?;
        let json: Value = serde_json::from_str(&output)?;

        if let Value::Array(monitors) = json {
            return Ok(monitors);
        }

        Err(Error::msg(
            "Failed to parse json from `hyprctl monitors -j`",
        ))
    }
}
