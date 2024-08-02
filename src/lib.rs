//! #Yew Google Material
//! 
//! `yew-google-material` is a very simple crate to use google materials `https://fonts.google.com/`(now only icons are availible)
//! 
//! Unfortunately a lot of crates with materials, icons for yew framework are depricated and use depricated javascript code. 
//! Here I plan to use only Rust code to connect yew with `https://fonts.google.com/` library. 
//! 
//! In the future I hope to add more posibilities with buttons, dialogs, animation, etc.
//!
//! To use Icons with the help of this crate you need to add some additional code to `index.html` inside `<head></head>`
//! There are three types of Icons in `https://fonts.google.com/icons`
//! Add this if you want to use icons Outlined style:
//! ```
//! <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! ```
//! To use icons Rounded style:
//! ```
//! <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! ```
//! To use icons Sharp style:
//! ```
//! <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Sharp:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! ```
//! Then choose on of the icons in the catalog of `<https://fonts.google.com/icons>`, i.e. 'search', 'star', 'menu', etc. and see example below. 
//! #Examples
//! ```
//! // with <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! 
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! <Icon 
//!     icon="search" 
//!     icon_style={IconStyle::Outlined} 
//!     fill=true
//!     wght="200"
//!     grade="0"
//!     opsz="24"
//!     color="#fff"
//!     width="1.5em"
//!     height="1.5em"
//! />
//! ```
//! Or you can add an icon with default options with another icons style:
//! ```
//! // with <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! 
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! <Icon 
//!     icon="star" 
//!     icon_style={IconStyle::Rounded} 
//! />
//! ```

pub mod icons;
pub mod prelude;


#[derive(Debug, PartialEq, Clone)]
pub enum IconStyle {
    Outlined,
    Rounded,
    Sharp,
}