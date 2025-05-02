use clap::{Parser, Subcommand};

use super::{app::AppResult, config::Config, hyprland::Hyprland};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
/// Hyprland Virtual Desktop Manager - Control Virtual Desktops In Hyprland On Multiple Monitors
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Initialize workspaces to match expected setup
    InitalizeWorkspaces,
    /// Change focus
    Focus {
        #[command(subcommand)]
        focus_command: FocusCommand,
    },
    /// Move current window
    MoveWindow {
        #[command(subcommand)]
        move_window_command: MoveWindowCommand,
    },
}

impl Command {
    pub fn run(&self, config: Config) -> AppResult<()> {
        let hyprland = Hyprland::new(config);

        match self {
            Command::InitalizeWorkspaces => hyprland.initialize_workspaces(),
            Command::Focus { focus_command } => focus_command.run(hyprland),
            Command::MoveWindow {
                move_window_command,
            } => move_window_command.run(hyprland),
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum FocusCommand {
    /// Focus a specific monitor
    Monitor {
        /// The key (set in your config) associated with the monitor you would like to focus
        key: String,
    },
    /// Focus the next workspace on the currently focused monitor
    NextWorkspace,
    /// Focus the previous workspace on the currently focused monitor
    PrevWorkspace,
    /// Focus a specific workspace on current focused monitor
    Workspace {
        /// A number between 1 and 5 that represents the workspace position on your current monitor
        order_num: u64,
    },
}

impl FocusCommand {
    pub fn run(&self, hyprland: Hyprland) -> AppResult<()> {
        match self {
            FocusCommand::Monitor { key } => hyprland.focus_monitor(key.to_string()),
            FocusCommand::NextWorkspace => hyprland.next_workspace(),
            FocusCommand::PrevWorkspace => hyprland.prev_workspace(),
            FocusCommand::Workspace { order_num } => hyprland.focus_workspace(order_num.to_owned()),
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MoveWindowCommand {
    /// Move a window to a specific monitor
    Monitor { key: String },
    /// Move a window to the next workspace on the currently focused monitor
    NextWorkspace,
    /// Move a window to the previous workspace on the currently focused monitor
    PrevWorkspace,
}

impl MoveWindowCommand {
    pub fn run(&self, hyprland: Hyprland) -> AppResult<()> {
        match self {
            MoveWindowCommand::Monitor { key } => hyprland.move_window_to_monitor(key.to_string()),
            MoveWindowCommand::NextWorkspace => hyprland.move_window_to_next_workspace(),
            MoveWindowCommand::PrevWorkspace => hyprland.move_window_to_prev_workspace(),
        }
    }
}
