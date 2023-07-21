mod ui;
mod data;
mod events;
mod logic;

use vizia::prelude::*;
use ui::ui_builder;
use data::AppData;

fn main() {
    Application::new(|cx|{

        // Add the stylesheet to the app
        cx.add_stylesheet(include_style!("src/styles/style.css")).expect("Failed to load stylesheet");

        // Build the data into the application
        AppData{
            password: "".to_string(),
            password_length: "".to_string(),
            special_chars: "".to_string(),
            numeric_chars: "".to_string()
        }.build(cx);
        
        // Contents
        ui_builder(cx);
    })
    .title("Password Generator")
    .inner_size((900, 300))
    .run();    
}
