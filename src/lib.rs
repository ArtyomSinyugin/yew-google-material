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
    pub use crate::buttons::GButton;
    pub use crate::GButtonStyle;
}

