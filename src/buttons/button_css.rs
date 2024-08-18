use std::str::FromStr;

use color_art::Color;
use stylist::Style;
use yew::prelude::*;

use crate::GButtonStyle;

#[allow(non_upper_case_globals)]
pub(super) fn input_style(
    style: &GButtonStyle,
    id: &AttrValue,
    only_icon: bool,
    g_init: &AttrValue,
    font_size: &AttrValue,
    height: &AttrValue,
    width: &Option<AttrValue>,
    background_color: &AttrValue,
    mut text_color: AttrValue,
    outlined_border_color: &Option<AttrValue>,
    border_radius: &AttrValue,
    disabled: bool,
    has_icon: bool,
    trailing_icon: bool,
    dark_theame: bool,
) -> Style {
    let button_background_color = Color::from_str(&background_color).unwrap();

    let width: AttrValue = if width.is_some() { 
        AttrValue::from(format!("width: {};", width.clone().unwrap()))
     } else { AttrValue::default() };

     let padding_left: &str;
     let padding_right: &str;
     let mut icon_align = "1.14em".to_string();
     if has_icon && !only_icon {
        if trailing_icon {
            padding_left = "1.71em";
            padding_right = "3em";
        } else {
            padding_left = "3em";
            padding_right = "1.71em";
        }
     } else if !only_icon {
        padding_left = "1.71em";
        padding_right = "1.71em";
     } else {
        padding_left = "1.34em";
        padding_right = "1.34em";
        icon_align = "0.77em".to_string();
     }
    
    let bsc: i16 = if dark_theame { 255 } else { 0 };
    let mut hover_shadow: String = String::new();
    let mut active_shadow: String = String::new();
    let background_color: String;
    let mut background_color_animation_from: String = String::new();
    let mut background_color_animation_to: String = String::new();
    let mut button_background_color_on_hover = String::new();
    let mut outlined_border_color_set: String = String::new();
    match style {
        GButtonStyle::Elevated => {
            if !disabled {
                let background_color_animation: Color;
                if button_background_color.lightness() > 0.5 {
                    button_background_color_on_hover = format!("background-color: {};", button_background_color.shade(0.9));                    
                    background_color_animation = Color::new(0, 0, 0, 1.0);
                } else {
                    button_background_color_on_hover = format!("background-color: {};", button_background_color.tint(0.9));                    
                    background_color_animation = Color::new(255, 255, 255, 1.0);
                }
                hover_shadow = format!("box-shadow: 0 3px 6px rgba({bsc},{bsc},{bsc},0.16), 0 3px 6px rgba({bsc},{bsc},{bsc},0.23);");
                active_shadow = format!("box-shadow: 0 2px 4px rgba({bsc},{bsc},{bsc},0.12), 0 1px 3px rgba({bsc},{bsc},{bsc},0.24);");
                background_color = format!("background-color: {button_background_color};");
                background_color_animation_from = format!("background-color: {};", background_color_animation.fade(0.1));
                background_color_animation_to = format!("background-color: {};", background_color_animation.fade(0.3));
            } else {
                background_color = format!("background-color: rgba(29, 27, 32, 0.12);");
            }
        },
        GButtonStyle::Filled => {
            if !disabled {
                let background_color_animation: Color;
                if button_background_color.lightness() > 0.5 {
                    button_background_color_on_hover = format!("background-color: {};", button_background_color.shade(0.9));                    
                    background_color_animation = Color::new(0, 0, 0, 1.0);
                } else {
                    button_background_color_on_hover = format!("background-color: {};", button_background_color.tint(0.9));                    
                    background_color_animation = Color::new(255, 255, 255, 1.0);
                }
                hover_shadow = format!("box-shadow: 0 2px 4px rgba({bsc},{bsc},{bsc},0.12), 0 1px 3px rgba({bsc},{bsc},{bsc},0.24);");
                active_shadow = format!("box-shadow: 0 0 0 rgba({bsc},{bsc},{bsc},0.0);");
                background_color = format!("background-color: {button_background_color};");
                background_color_animation_from = format!("background-color: {};", background_color_animation.fade(0.1));
                background_color_animation_to = format!("background-color: {};", background_color_animation.fade(0.3));
            } else {
                background_color = format!("background-color: rgba(29, 27, 32, 0.12);");
            }
        },
        GButtonStyle::Outlined => {
            background_color = String::from("background-color: unset;");
            if !disabled {
                button_background_color_on_hover = format!("background-color: {};", button_background_color.fade(0.1));
                background_color_animation_from = format!("background-color: {};", button_background_color.fade(0.1));
                background_color_animation_to = format!("background-color: {};", button_background_color.fade(0.3));
                outlined_border_color_set = if let Some(color) = outlined_border_color {
                    format!("border: 0.0714em solid {};", color)
                } else {
                    format!("border: 0.0714em solid #79747E;")
                };
            } else {
                outlined_border_color_set = String::from("border: 0.0714em solid rgba(29, 27, 32, 0.12);");
            }
        },
        GButtonStyle::Text => {
            background_color = String::from("background-color: unset;");
            if !disabled {
                button_background_color_on_hover = format!("background-color: {};", button_background_color.fade(0.1));                background_color_animation_from = format!("background-color: {};", button_background_color.fade(0.1));
                background_color_animation_to = format!("background-color: {};", button_background_color.fade(0.3));
            }
        },
    }
    if disabled {
        text_color = AttrValue::from("rgba(29, 27, 32, 0.38)");
    }
    let mut style_str = format!(
        r#"
            div#{g_init} {{
                display: inline-block;
                font-size: {font_size};
                position: relative;
                color: {text_color};
                line-height: 0;
                margin: 0;
                padding: 0;
            }}

            button#{id} {{
                height: {height};
                {width}
                padding: 0.7em {padding_right} 0.7em {padding_left};
                border: none;
                line-height: 1.42em;
                font-size: inherit;
                font-weight: 500;
                border-radius: {border_radius};
                {background_color}
                outline: none;
                cursor: pointer;
                color: inherit;
                position: relative;
                overflow: hidden;
                transition: box-shadow ease-out 0.3s;
                {outlined_border_color_set}
            }}

            button#{id}:hover, button#{id}:focus {{
                {hover_shadow}
                {button_background_color_on_hover}
            }}

            button#{id}, button#{id}:active {{
                {active_shadow}
            }}

            button#{id} > span#g_init_span{id} {{
                position: absolute;
                border-radius: 50%;
                pointer-events: auto;
                width: 200%;
                aspect-ratio: 1;
                margin-top: -100%;
                margin-left: -100%;
                animation: ripple ease-in-out 0.6s forwards;
            }}

            @keyframes ripple {{
                from {{
                    transform: scale(0.2);
                    {background_color_animation_from}
                }}
            
                to {{
                    transform: scale(2);
                    {background_color_animation_to}
                }}
            }}
        "#
    );

    if has_icon {
        let has_icon: String;
        if trailing_icon {
            has_icon = format!(r#"
            #{g_init} gicon.g_has_trailing_icon {{
                display: block;
                position: absolute;
                top: 50%;
                transform: translateY(-50%);
                right: {icon_align};
                pointer-events: none;
            }}
    
            #{g_init} gicon.g_has_trailing_icon > div > span {{
                font-size: 1.29em !important;  
                color: inherit !important;
            }}
            "#);
        } else {
            has_icon = format!(r#"
            #{g_init} gicon.g_has_leading_icon {{
                display: block;
                position: absolute;  
                top: 50%;
                transform: translateY(-50%);
                left: {icon_align};
                pointer-events: none;
            }}
    
            #{g_init} gicon.g_has_leading_icon > div > span {{
                font-size: 1.29em !important;  
                color: inherit !important;
            }}
            "#);
        }
        style_str.push_str(has_icon.as_str());
    }

    Style::new(style_str).expect("Failed to create style for input field")
}