//! # GIcon
//! helps to connect yew app with `https://fonts.google.com/icons`
//!
//! To use icons with the help of this crate you need to add some additional code to `index.html` inside `<head></head>`
//! 
//! There are three types of Icons. Add this if you want to use icons Outlined style:
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
//! ## Examples
//! ```
//! // with <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! 
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! <GIcon 
//!     size="16px"
//!     transition="all 0.2s"
//!     icon="search" 
//!     icon_style={GIconStyle::Outlined} 
//!     fill=true
//!     wght="200"
//!     grade="0"
//!     opsz="24"
//!     color="#fff"
//! />
//! ```
//! Note, you that you can animate icon attributes with transition. The default value is "unset", but for animation it is recomended to set "all 0.2s" or as you wish.
//! 
//! Or you can add an icon with default options with another icons style:
//! ```
//! // with <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
//! 
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! <GIcon 
//!     icon="star" 
//!     icon_style={GIconStyle::Rounded} 
//! />
//! ```

use stylist::Style;
use yew::prelude::*;
use crate::GIconStyle;
use web_sys::Element;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct GIconProps {
    #[prop_or_default]
    pub icon: AttrValue,
    pub icon_style: GIconStyle,
    #[prop_or_else(|| false )]
    pub fill: bool,
    #[prop_or_else(|| AttrValue::from("300"))]
    pub wght: AttrValue,
    #[prop_or_else(|| AttrValue::from("100"))]
    pub grade: AttrValue,
    #[prop_or_else(|| AttrValue::from("24"))]
    pub opsz: AttrValue,
    #[prop_or_else(|| AttrValue::from(r#"inherit"#))]
    pub color: AttrValue,
    #[prop_or_else(|| AttrValue::from(r#"24px"#))]
    pub size: AttrValue,
    #[prop_or_else(|| AttrValue::from("unset"))]
    pub transition: AttrValue,
    #[prop_or_default]
    pub leading_icon: bool, 
    #[prop_or_default]
    pub trailing_icon: bool, 
}

#[function_component(GIcon)]
pub fn icon(props: &GIconProps) -> Html {
    let universal_slyle = match props.icon_style {
        GIconStyle::Outlined => "material-symbols-outlined",
        GIconStyle::Rounded => "material-symbols-rounded",
        GIconStyle::Sharp => "material-symbols-sharp",
    };
    let fill = if props.fill { 1_u8 } else { 0_u8 };
    let wght = {
        match props.wght.parse::<u16>() {
            Ok(x) => {if x >= 100 && x <= 700 {x} else {panic!("Wrong Google Material Icon wght setting")}},
            Err(_) => panic!("Wrong Google Material Icon wght setting"),
        }
    };
    let grade = {
        match props.grade.parse::<i16>() {
            Ok(x) => {if x >= -50 && x <= 200 {x} else {panic!("Wrong Google Material Icon grade setting")}},
            Err(_) => panic!("Wrong Google Material Icon grade setting"),
        }
    };
    let opsz = {
        match props.opsz.parse::<u8>() {
            Ok(x) => {if x >= 20 && x <= 48 {x} else {panic!("Wrong Google Material Icon opsz setting")}},
            Err(_) => panic!("Wrong Google Material Icon opsz setting"),
        }
    };

    let color = props.color.clone();
    let transition = props.transition.clone();
    let size = props.size.clone();
    let leading_icon = props.leading_icon.clone();
    let trailing_icon = props.trailing_icon.clone();

    let style_str = format!(
        r#"
            .{universal_slyle} {{
                display: block;
                transition: {transition};
                color: {color};
                font-size: {size};
                font-variation-settings:
                'FILL' {fill},
                'wght' {wght},
                'GRAD' {grade},
                'opsz' {opsz}
                }}
        "#
    );

    let node_ref = NodeRef::default();
    let node_ref_clone = node_ref.clone();

    let style = Style::new(style_str).expect("Failed to create style");

    use_effect(move || {
        if leading_icon || trailing_icon {
            let gicon = node_ref_clone.cast::<Element>().unwrap();
            if leading_icon {
                gicon.set_class_name("g_has_leading_icon");
            }
            if trailing_icon {
                gicon.set_class_name("g_has_trailing_icon");
            }
        };
        ()
    });

    html! {
        <gicon ref={node_ref} style="line-height: 0">
            <div class={style} style="line-height: 0">
                <span class={universal_slyle}>{props.icon.clone()}</span>
            </div>
        </gicon>
    }
}