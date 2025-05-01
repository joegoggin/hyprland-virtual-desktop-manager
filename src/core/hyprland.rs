use anyhow::Error;
use serde_json::Value;

use crate::utils::{terminal_command::TerminalCommand, value::GetOrDefault};

use super::{app::AppResult, config::Config, monitor::Monitor};

#[derive(Debug)]
pub struct Hyprland {
    config: Config,
}

impl Hyprland {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn initialize_workspaces(&self) -> AppResult<()> {
        for (_, monitor) in &self.config.monitors {
            let command = format!("hyprctl dispatch workspace {}", monitor.min_workspace_id);
            TerminalCommand::new(command).run()?;

            let command = format!(
                "hyprctl dispatch movecurrentworkspacetomonitor {}",
                monitor.name
            );
            TerminalCommand::new(command).run()?;
        }

        Ok(())
    }

    pub fn next_workspace(&self) -> AppResult<()> {
        let monitor = self.get_active_monitor()?;
        let workspace_id = self.get_active_workspace_id(monitor.id)?;
        let mut new_workspace_id = workspace_id + 1;

        if new_workspace_id > monitor.max_workspace_id {
            new_workspace_id = monitor.min_workspace_id;
        }

        let command = format!("hyprctl dispatch workspace {}", new_workspace_id);
        TerminalCommand::new(command).run()?;

        Ok(())
    }

    pub fn focus_monitor(&self, key: String) -> AppResult<()> {
        let monitor = self.config.monitors.get(&key);

        if let Some(monitor) = monitor {
            let active_workspace_id = self.get_active_workspace_id(monitor.id)?;
            let command = format!("hyprctl dispatch workspace {}", active_workspace_id);

            TerminalCommand::new(command).run()?;
        }

        Ok(())
    }

    fn get_monitor_values(&self) -> AppResult<Vec<Value>> {
        let output = TerminalCommand::new("hyprctl monitors -j").run_with_output()?;
        let json: Value = serde_json::from_str(&output)?;
        let mut monitor_values: Vec<Value> = vec![];

        if let Value::Array(values) = json {
            monitor_values = values;
        }

        Ok(monitor_values)
    }

    fn get_workspace_values(&self) -> AppResult<Vec<Value>> {
        let output = TerminalCommand::new("hyprctl workspaces -j").run_with_output()?;
        let json: Value = serde_json::from_str(&output)?;
        let mut worspace_values: Vec<Value> = vec![];

        if let Value::Array(values) = json {
            worspace_values = values;
        }

        Ok(worspace_values)
    }

    fn get_active_monitor(&self) -> AppResult<Monitor> {
        let active_monitor_id = self.get_active_monitor_id()?;

        for (_, monitor) in &self.config.monitors {
            if monitor.id == active_monitor_id {
                return Ok(monitor.clone());
            }
        }

        Err(Error::msg("Unable to find active monitor"))
    }

    fn get_active_monitor_id(&self) -> AppResult<u64> {
        let monitor_values = self.get_monitor_values()?;
        let mut active_monitor_id = 0;

        for value in monitor_values {
            let is_active = value.get_bool_or_default("focused");

            if is_active {
                active_monitor_id = value.get_number_or_default("id");
            }
        }

        Ok(active_monitor_id)
    }

    fn get_active_workspace_id(&self, monitor_id: u64) -> AppResult<u64> {
        let monitor_values = self.get_monitor_values()?;
        let mut active_workspace_id = 0;

        for value in monitor_values {
            let id = value.get_number_or_default("id");

            if id == monitor_id {
                if let Some(active_workspace) = value.get("activeWorkspace") {
                    active_workspace_id = active_workspace.get_number_or_default("id");
                }

                break;
            }
        }

        Ok(active_workspace_id)
    }
}
