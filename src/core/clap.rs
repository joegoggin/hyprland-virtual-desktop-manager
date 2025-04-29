use clap::{Parser, Subcommand};

use super::app::AppResult;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
/// Hyprland Virtual Desktop Manager - Control Virtual Desktops In Hyprland On Multiple Monitors
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Setup monitor configuration
    Config,
    /// Focus a specific monitor
    FocusMonitor { monitor_id: u8 },
    /// Go to the next workspace on the currently focused monitor
    NextWorkspace,
    /// Go to the previous workspace on the currently focused monitor
    PrevWorkspace,
    /// Move a window to a specific monitor
    MoveWindowToMonitor { monitor_id: u8 },
    /// Move a window to the next workspace on the currently focused monitor
    MoveWindowToNextWorkspace,
    /// Move a window to the previous workspace on the currently focused monitor
    MoveWindowToPrevWorkspace,
    /// Close a window and remove virtual desktop if empty
    Close,
}

impl Command {
    pub fn run(&self) -> AppResult<()> {
        match self {
            Command::Config => println!("Config"),
            Command::FocusMonitor { monitor_id } => {
                println!("FocusMonitor - monitor_id: {}", monitor_id)
            }
            Command::NextWorkspace => println!("NextWorkspace"),
            Command::PrevWorkspace => println!("PrevWorkspace"),
            Command::MoveWindowToMonitor { monitor_id } => {
                println!("MoveWindowToMonitor - monitor_id: {}", monitor_id)
            }
            Command::MoveWindowToNextWorkspace => println!("MoveWindowToNextWorkspace"),
            Command::MoveWindowToPrevWorkspace => println!("MoveWindowToPrevWorkspace"),
            Command::Close => println!("Close"),
        }

        Ok(())
    }
}
