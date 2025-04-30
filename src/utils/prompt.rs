use std::io::stdin;

use colorized::{Colors, colorize_print, colorize_println};

use crate::core::app::AppResult;

pub struct Prompt {
    message: String,
}

impl Prompt {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn yes_or_no(&self) -> AppResult<bool> {
        colorize_println(format!("{}\n", self.message), Colors::BlueFg);
        colorize_print("Enter '", Colors::BlueFg);
        colorize_print("y", Colors::GreenFg);
        colorize_print("' for yes or '", Colors::BlueFg);
        colorize_print("n", Colors::RedFg);
        colorize_println("' for no\n", Colors::BlueFg);

        let response = self.get_response()?;

        if response.trim() == "y" || response.trim() == "Y" {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn string(&self) -> AppResult<String> {
        colorize_println(format!("{}\n", self.message), Colors::BlueFg);

        self.get_response()
    }

    fn get_response(&self) -> AppResult<String> {
        let mut response = String::new();

        stdin().read_line(&mut response)?;

        Ok(response[0..response.len() - 1].to_string())
    }
}
