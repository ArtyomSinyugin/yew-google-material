//! # Yew Google Material
//! 
//! `yew-google-material` is a very simple crate to use some of google materials and `https://fonts.google.com/`
//! 
//! Unfortunately a lot of crates with materials, icons for yew framework are depricated and use depricated javascript code.
//!  
//! Here I use only Rust code to add some design features for yew. 
//! 
//! Now only buttons, text fields and icons are available.
//! Buttons and text fields are not the same as one in google material web, but very similar to them. 
//! 
//! See more information in `GButton`, `GIcon` and `GTextInput` modules below.

use yew::AttrValue;

pub mod icons;
pub mod input_text;
pub mod buttons;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum GIconStyle {
    #[default]
    Outlined,
    Rounded,
    Sharp,
}

#[derive(Default, PartialEq)]
pub enum GInputStyle {
    #[default]
    Outlined,
    Filled,
}

#[derive(PartialEq, Default, Clone)]
pub enum GButtonStyle {
    Elevated,
    #[default]
    Filled, 
    Outlined,
    Text,
}

pub mod prelude {
    pub use crate::icons::GIcon;
    pub use crate::GIconStyle;
    pub use crate::input_text::{GTextInput, GInputEvent};
    pub use crate::GInputStyle;
    pub use crate::buttons::{GButton, DependsOn};
    pub use crate::GButtonStyle;
}

fn parse_number_and_unit(input: AttrValue) -> (f64, String) {
    let input = input.as_str();
    let pos = input
        .find(|c: char| !c.is_digit(10) && c != '.' && c != '+' && c != '-');

    match pos {
        Some(pos) => {
            let (number_str, unit) = input.split_at(pos);
            let number = match number_str.parse::<f64>() {
                Ok(number) => number,
                Err(_) => {
                    return (3.5, "em".to_string());
                },
            };
            (number, unit.to_string())
        },
        None => {
            let number = match input.parse::<f64>() {
                Ok(number) => number,
                Err(_) => {
                    return (3.5, "em".to_string());
                },
            };
            (number, "".to_string())
        }, 
    }
}