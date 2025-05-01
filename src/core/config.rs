use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process,
};

use colorized::{Colors, colorize_print, colorize_println};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    core::hyprctl::Hyprctl,
    utils::{
        directory::get_home_dir, prompt::Prompt, terminal_command::TerminalCommand,
        value::GetOrDefault,
    },
};

use super::{app::AppResult, monitor::Monitor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub monitors: HashMap<String, Monitor>,
    pub main_monitor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            monitors: HashMap::new(),
            main_monitor: String::new(),
        }
    }
}

impl Config {
    pub fn new(monitors: HashMap<String, Monitor>, main_monitor_key: String) -> Self {
        Self {
            monitors,
            main_monitor: main_monitor_key,
        }
    }

    pub fn load_from_file(&mut self) -> AppResult<()> {
        let path = self.get_file_path()?;
        let mut data = String::new();
        let mut file = File::open(Path::new(&path))?;
        file.read_to_string(&mut data)?;

        let config: Config = serde_json::from_str::<Config>(&data)?;

        self.monitors = config.monitors;

        Ok(())
    }

    pub async fn prompt_user(&mut self) -> AppResult<()> {
        TerminalCommand::clear()?;

        colorize_println("Warning: No config file found\n", Colors::YellowFg);

        let prompt = Prompt::new("Would you like to create a new config?");

        if !prompt.yes_or_no()? {
            TerminalCommand::clear()?;
            process::exit(0);
        }

        let monitor_values = Hyprctl::monitors()?;
        let mut monitor_vec: Vec<Monitor> = vec![];

        for (i, value) in monitor_values.iter().enumerate() {
            let id = value.get_number_or_default("id");
            let name = value.get_string_or_default("name");
            let description = value.get_string_or_default("description");
            let mut min_workspace_id: u64 = 1;

            if i > 0 {
                if let Some(prev_monitor) = monitor_vec.get(i - 1) {
                    min_workspace_id = prev_monitor.max_workspace_id + 1;
                }
            }

            let monitor = Monitor::new(
                id,
                name,
                description,
                min_workspace_id,
                min_workspace_id + 4,
            );

            monitor_vec.push(monitor);
        }

        TerminalCommand::clear()?;

        colorize_println(
            format!("We have detected {} monitors:", monitor_vec.len()),
            Colors::BlueFg,
        );

        for monitor in &monitor_vec {
            colorize_print("\nname: ", Colors::GreenFg);
            colorize_println(format!("{}", monitor.name), Colors::Reset);
            colorize_print("description: ", Colors::GreenFg);
            colorize_println(format!("{}\n", monitor.description), Colors::Reset);
        }

        colorize_println(
            "Set a key for each monitor (This key will be used to identify your monitor while using this tool)",
            Colors::BlueFg,
        );

        let mut monitors: HashMap<String, Monitor> = HashMap::new();

        for monitor in monitor_vec {
            let prompt = Prompt::new(format!("\nChoose a key for '{}'", monitor.name));
            let key = prompt.string()?;

            if self.main_monitor == "" {
                let prompt = Prompt::new("\nIs this your main monitor?");

                if prompt.yes_or_no()? {
                    self.main_monitor = key.clone();
                }
            }

            monitors.insert(key, monitor);
        }

        println!("{:#?}", monitors);

        self.monitors = monitors;
        self.write_to_file()?;

        colorize_println("\nConfig successfully created!\n", Colors::GreenFg);

        Ok(())
    }

    pub fn write_to_file(&self) -> AppResult<()> {
        let data = serde_json::to_string_pretty(self)?;
        let file_path = self.get_file_path()?;

        if let Some(parent) = Path::new(&file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(file_path.clone())?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    fn get_file_path(&self) -> AppResult<String> {
        let home_dir = get_home_dir()?;

        Ok(format!("{}/.config/hyprland-vdm/config.json", home_dir))
    }
}
