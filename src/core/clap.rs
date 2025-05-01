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
    /// Setup monitor configuration
    Config,
    /// Focus a specific monitor
    FocusMonitor { key: String },
    /// Go to the next workspace on the currently focused monitor
    NextWorkspace,
    /// Go to the previous workspace on the currently focused monitor
    PrevWorkspace,
    /// Move a window to a specific monitor
    MoveWindowToMonitor { key: String },
    /// Move a window to the next workspace on the currently focused monitor
    MoveWindowToNextWorkspace,
    /// Move a window to the previous workspace on the currently focused monitor
    MoveWindowToPrevWorkspace,
    /// Close a window and remove virtual desktop if empty
    Close,
}

impl Command {
    pub fn run(&self, config: Config) -> AppResult<()> {
        let hyprland = Hyprland::new(config);

        match self {
            Command::InitalizeWorkspaces => hyprland.initialize_workspaces()?,
            Command::Config => println!("Config"),
            Command::FocusMonitor { key } => hyprland.focus_monitor(key.to_string())?,
            Command::NextWorkspace => hyprland.next_workspace()?,
            Command::PrevWorkspace => println!("PrevWorkspace"),
            Command::MoveWindowToMonitor { key } => {
                println!("MoveWindowToMonitor - key: {}", key)
            }
            Command::MoveWindowToNextWorkspace => println!("MoveWindowToNextWorkspace"),
            Command::MoveWindowToPrevWorkspace => println!("MoveWindowToPrevWorkspace"),
            Command::Close => println!("Close"),
        }

        Ok(())
    }
}
