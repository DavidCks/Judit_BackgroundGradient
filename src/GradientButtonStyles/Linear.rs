use rusty_css::*;
use append_to_string::*;
use bevy_reflect::{ Reflect };

#[derive(Reflect)]
pub struct RCSS {
    background_image: String,
    background_color: String,
}

impl Style for RCSS {
    fn create() -> Self {
        append_to_string!(
            Self {
                background_image: "linear-gradient(90deg, #eeeeee, #3f3f3f)",
                background_color: "initial",
            }
        )
    }
}