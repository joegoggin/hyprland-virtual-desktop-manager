use anyhow::Error;

use crate::utils::value::GetOrDefault;

use super::{app::AppResult, config::Config, hyprctl::Hyprctl, monitor::Monitor};

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
            Hyprctl::create_workspace(monitor.min_workspace_id, monitor.name.to_string())?;
        }

        Ok(())
    }

    pub fn next_workspace(&self) -> AppResult<()> {
        let next_workspace_id = self.get_next_workspace_id()?;

        Hyprctl::go_to_workspace(next_workspace_id)?;

        Ok(())
    }

    pub fn prev_workspace(&self) -> AppResult<()> {
        let prev_workspace_id = self.get_prev_workspace_id()?;

        Hyprctl::go_to_workspace(prev_workspace_id)?;

        Ok(())
    }

    pub fn focus_monitor(&self, key: String) -> AppResult<()> {
        let monitor = self.config.monitors.get(&key);

        if let Some(monitor) = monitor {
            let active_workspace_id = self.get_active_workspace_id(monitor.id)?;

            Hyprctl::go_to_workspace(active_workspace_id)?;
        }

        Ok(())
    }

    pub fn move_window_to_monitor(&self, key: String) -> AppResult<()> {
        let monitor = self.config.monitors.get(&key);

        if let Some(monitor) = monitor {
            let active_workspace_id = self.get_active_workspace_id(monitor.id)?;

            Hyprctl::move_to_workspace(active_workspace_id)?;
        }

        Ok(())
    }

    pub fn move_window_to_next_workspace(&self) -> AppResult<()> {
        let monitor = self.get_active_monitor()?;
        let workspace_id = self.get_active_workspace_id(monitor.id)?;
        let next_workspace_id = self.get_next_workspace_id()?;

        Hyprctl::move_to_workspace(next_workspace_id)?;
        Hyprctl::go_to_workspace(workspace_id)?;

        Ok(())
    }

    pub fn move_window_to_prev_workspace(&self) -> AppResult<()> {
        let monitor = self.get_active_monitor()?;
        let workspace_id = self.get_active_workspace_id(monitor.id)?;
        let prev_workspace_id = self.get_prev_workspace_id()?;

        Hyprctl::move_to_workspace(prev_workspace_id)?;
        Hyprctl::go_to_workspace(workspace_id)?;

        Ok(())
    }

    pub fn focus_workspace(&self, order_num: u64) -> AppResult<()> {
        let ordered_workspace_id = self.get_orderd_workspace_id(order_num)?;

        Hyprctl::go_to_workspace(ordered_workspace_id)?;

        Ok(())
    }

    pub fn move_window_to_workspace(&self, order_num: u64) -> AppResult<()> {
        let monitor_id = self.get_active_monitor_id()?;
        let workspace_id = self.get_active_workspace_id(monitor_id)?;
        let ordered_workspace_id = self.get_orderd_workspace_id(order_num)?;

        Hyprctl::move_to_workspace(ordered_workspace_id)?;
        Hyprctl::go_to_workspace(workspace_id)?;

        Ok(())
    }

    fn get_orderd_workspace_id(&self, order_num: u64) -> AppResult<u64> {
        if order_num < 1 || order_num > 5 {
            return Err(Error::msg("Order number must be between 1 and 5"));
        }

        let monitor = self.get_active_monitor()?;
        let ordered_workspace_id = (order_num - 1) + monitor.min_workspace_id;

        Ok(ordered_workspace_id)
    }

    fn get_next_workspace_id(&self) -> AppResult<u64> {
        let monitor = self.get_active_monitor()?;
        let workspace_id = self.get_active_workspace_id(monitor.id)?;
        let mut next_workspace_id = workspace_id + 1;

        if next_workspace_id > monitor.max_workspace_id {
            next_workspace_id = monitor.min_workspace_id;
        }

        Ok(next_workspace_id)
    }

    fn get_prev_workspace_id(&self) -> AppResult<u64> {
        let monitor = self.get_active_monitor()?;
        let workspace_id = self.get_active_workspace_id(monitor.id)?;
        let mut next_workspace_id = workspace_id - 1;

        if next_workspace_id < monitor.min_workspace_id {
            next_workspace_id = monitor.max_workspace_id;
        }

        Ok(next_workspace_id)
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
        let monitor_values = Hyprctl::monitors()?;
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
        let monitor_values = Hyprctl::monitors()?;
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
