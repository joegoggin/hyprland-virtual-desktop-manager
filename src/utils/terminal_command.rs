use std::process::Command;

use anyhow::Error;

use crate::core::app::AppResult;

pub struct TerminalCommand {
    command: String,
    args: Vec<String>,
}

impl TerminalCommand {
    pub fn new(full_command: impl Into<String>) -> Self {
        let full_command: String = full_command.into();
        let full_command_split: Vec<&str> = full_command.split(' ').collect();
        let mut command = String::new();
        let mut args: Vec<String> = vec![];

        if full_command_split.len() > 0 {
            command = full_command_split[0].into()
        }

        for i in 1..full_command_split.len() {
            let arg: String = full_command_split[i].into();

            args.push(arg);
        }

        Self { command, args }
    }

    pub fn run(&self) -> AppResult<()> {
        let status = self.get_command().status()?;

        if !status.success() {
            let error_message = format!("The following command failed `{}`", self.command);

            return Err(Error::msg(error_message));
        }

        Ok(())
    }

    pub fn run_with_output(&self) -> AppResult<String> {
        let output = self.get_command().output();

        if let Ok(output) = output {
            if !output.status.success() {
                let error_message = format!(
                    "The following terminal command failed `{}`\n\n{}",
                    self.command,
                    &String::from_utf8_lossy(&output.stderr)
                );

                return Err(Error::msg(error_message));
            }

            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }

        Ok(String::new())
    }

    pub fn clear() -> AppResult<()> {
        let clear_command = Self::new("clear");

        clear_command.run()
    }

    fn get_command(&self) -> Command {
        let mut command = Command::new(&self.command);

        for arg in &self.args {
            command.arg(&arg);
        }

        command
    }
}
