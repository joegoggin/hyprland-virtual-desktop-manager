use serde_json::Value;

use crate::utils::{terminal_command::TerminalCommand, value::GetOrDefault};

use super::{app::AppResult, config::Config};

#[derive(Debug)]
pub struct Hyprland {
    config: Config,
}

impl Hyprland {
    pub fn new(config: Config) -> Self {
        Self { config }
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
