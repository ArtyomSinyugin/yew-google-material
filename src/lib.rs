//! # Yew Google Material
//! 
//! `yew-google-material` is a very simple crate to use some of google materials and `https://fonts.google.com/`
//! 
//! Unfortunately a lot of crates with materials, icons for yew framework are depricated and use depricated javascript code.
//!  
//! Here I plan to use only Rust code to add some design features for yew. 
//! 
//! Now only text fields and icons are available. Buttons in plans. 
//! Text fields are not the same as one in google material web, but very similar to them. 
//! 
//! See more information in `GIcon` and `GTextInput` modules below.

pub mod icons;
pub mod input_text;

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

pub mod prelude {
    pub use crate::GIconStyle;
    pub use crate::icons::GIcon;
    pub use crate::GInputStyle;
    pub use crate::input_text::{GTextInput, GInputEvent};
}

