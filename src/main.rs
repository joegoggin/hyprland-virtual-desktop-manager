use colorized::{Colors, colorize_println};
use hyprland_vdm::core::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    let result = app.run().await;

    if let Err(error) = result {
        let error_message = format!("Error: {}", error.to_string());

        colorize_println(error_message, Colors::RedFg);
    }
}
