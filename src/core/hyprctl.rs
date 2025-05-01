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

    pub fn create_workspace(workspace_id: u64, monitor_name: String) -> AppResult<()> {
        Self::go_to_workspace(workspace_id)?;

        let command = format!(
            "hyprctl dispatch movecurrentworkspacetomonitor {}",
            monitor_name
        );
        TerminalCommand::new(command).run()?;

        Ok(())
    }

    pub fn go_to_workspace(workspace_id: u64) -> AppResult<()> {
        let command = format!("hyprctl dispatch workspace {}", workspace_id);
        TerminalCommand::new(command).run()?;

        Ok(())
    }

    pub fn move_to_workspace(workspace_id: u64) -> AppResult<()> {
        let command = format!("hyprctl dispatch movetoworkspace {}", workspace_id);
        TerminalCommand::new(command).run()?;

        Ok(())
    }
}
