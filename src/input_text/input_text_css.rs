use stylist::Style;
use yew::prelude::*;

use crate::GInputStyle;

#[allow(non_upper_case_globals)]
pub(super) fn input_style(
    style: &GInputStyle,
    id: AttrValue,
    g_init: AttrValue,
    g_container: AttrValue,
    g_input_width: AttrValue, 
    g_input_height: Option<AttrValue>, 
    g_font_size: AttrValue, 
    g_input_border_radius: AttrValue, 
    g_input_border_color: AttrValue, 
    g_input_border_color_hover: AttrValue,
    g_input_border_focus_color: AttrValue, 
    mut g_label_background_color: AttrValue, 
    g_label_text_color: AttrValue, 
    g_align_supporting_text: AttrValue,
    g_supporting_text_color: Option<AttrValue>,
    no_asterisk: bool,
    has_leading_icon: bool,
    has_trailing_icon: bool,
    no_spinner: Option<bool>,
) -> Style {
    let g_supporting_text_color = if let Some(value) = g_supporting_text_color {
        AttrValue::from(format!("color: {value};"))
    } else {
        AttrValue::default()
    };
    let input_padding_left;
    let label_left;
    if has_leading_icon {
        input_padding_left = "2.75em";
        label_left = "2.75em";
    } else {
        input_padding_left = "1em";
        label_left = "1em";
    }
    let input_padding_right;
    if has_trailing_icon {
        input_padding_right = "2.75em";
    } else {
        input_padding_right = "0";
    }

    let g_container_margin: AttrValue;
    let input_border: AttrValue;
    let input_border_onfocus: AttrValue;
    let input_border_bottom_radius: AttrValue;
    let input_padding_top: AttrValue;
    let input_padding_bottom: AttrValue;
    let input_background_color: AttrValue;
    let input_background_color_onfocus: AttrValue;
    let input_background_color_onhover: AttrValue;
    let label_on_focus_left: AttrValue;
    let label_on_focus_top: AttrValue;

    if *style == GInputStyle::Outlined {
        g_container_margin = AttrValue::from("0.5em 0 0.2em");
        input_border = AttrValue::from("0.0625em solid");
        input_border_onfocus = AttrValue::from("0.1875em solid");
        input_border_bottom_radius = g_input_border_radius.clone();
        input_padding_top = AttrValue::from("1.1875em");
        input_padding_bottom = AttrValue::from("1.1875em");
        input_background_color = AttrValue::from("unset");
        input_background_color_onfocus = AttrValue::default();
        input_background_color_onhover = AttrValue::default();
        label_on_focus_left = AttrValue::from("0.75em");
        label_on_focus_top = AttrValue::from("-0.55em");
    } else {
        g_container_margin = AttrValue::from("0.5em 0 0.2em");
        input_border = AttrValue::from("0 none");
        input_border_onfocus = AttrValue::from("0 none");
        input_border_bottom_radius = AttrValue::from("0");
        if no_asterisk {
            input_padding_top = AttrValue::from("1.1875em");
            input_padding_bottom = AttrValue::from("1.1875em");
        } else {
            input_padding_top = AttrValue::from("1.25em");
            input_padding_bottom = AttrValue::from("0.5em");
        }
        input_background_color = AttrValue::from("whitesmoke");
        input_background_color_onfocus = AttrValue::from("background-color: #eeeeee;");
        input_background_color_onhover = AttrValue::from("background-color: #eeeeee;");
        g_label_background_color = AttrValue::from("transparent");
        if has_leading_icon {
            label_on_focus_left = AttrValue::from("3.5em");
        } else {
            label_on_focus_left = AttrValue::from("1.125em");
        }
        label_on_focus_top = AttrValue::from("0.5em");
    }

    let height: String;
    let label_top: String;
    let icon_top: String;
    if g_input_height.is_none() {
        height = "3.5em".to_string();
        label_top = "1.1875em".to_string();
        icon_top = "1em".to_string();
    } else {
        let (height_digit, height_text) = crate::parse_number_and_unit(g_input_height.clone().unwrap());
        height = g_input_height.unwrap().to_string();
        label_top = format!("{}{}", (height_digit / 2.95), height_text);
        icon_top = format!("{}{}", (height_digit / 3.5), height_text);
    }
    let mut style_str = format!(
        r#"
        #{g_init} {{
            font-size: {g_font_size}; 
            width: {g_input_width};    
            box-sizing: border-box;
            width: 100%;
        }}

        #{g_container} {{
            width: 100%;
            position: relative;
            margin: {g_container_margin};
            padding: 0;
        }}
        
        input#{id} {{
            width: 100%;
            height: {height};
            padding: {input_padding_top} {input_padding_right} {input_padding_bottom} {input_padding_left};
            margin: 0;
            transition: border 0.2s;
            line-height: 1em;
            border: {input_border};
            border-bottom: 0.0625em solid;
            border-color: {g_input_border_color};
            border-radius: {input_border_bottom_radius};
            border-top-left-radius: {g_input_border_radius};
            border-top-right-radius: {g_input_border_radius};
            background-color: {input_background_color};
            font-size: 1em;
            box-sizing: border-box;
            resize: vertical;
        }}

        input#{id}:hover {{
            border-color: {g_input_border_color_hover};
            {input_background_color_onhover}
        }}
        
        input#{id}:focus,
        input#{id}:not(:placeholder-shown) {{
            outline: none;
            border: {input_border_onfocus};
            border-bottom: 0.1875em solid;
            border-color: {g_input_border_focus_color};
            outline-color: {g_input_border_focus_color};
            overflow: hidden;
            {input_background_color_onfocus}
        }}

        input#{id}:not(:focus) {{
            border: {input_border};
            border-bottom: 0.0625em solid;
            border-color: {g_input_border_color};
        }}
        
        #{g_container} > label {{
            position: absolute;
            top: {label_top};
            left: {label_left};
            line-height: 1em;
            background-color: {g_label_background_color}; 
            padding: 0;
            font-size: 1em;
            color: {g_label_text_color};
            transition: 0.2s;
            pointer-events: none;
        }}

        .g_supporting_text_below_input_text_field {{
            text-align: {g_align_supporting_text};
            font-size: 0.75em;
            width: calc({g_input_width} - 2em);
            margin: 0;
            padding: 0 1em;
            {g_supporting_text_color}
        }}
        "#
    );
    

    if has_leading_icon {
        let has_leading_icon_str = format!(r#"
        #{g_init} gicon.g_has_leading_icon {{
            display: block;
            position: absolute;  
            top: {icon_top};
            left: 0.5625em;
            pointer-events: none;
        }}

        #{g_init} gicon.g_has_leading_icon > div > span {{
            font-size: 1.5em !important;  
        }}
        "#);
        style_str.push_str(has_leading_icon_str.as_str());
    } 

    if has_trailing_icon {
        let has_trailing_icon_str = format!(r#"
        #{g_init} gicon.g_has_trailing_icon {{
            display: block;
            position: absolute;
            top: {icon_top};
            right: 0.5625em;
            pointer-events: none;
        }}

        #{g_init} gicon.g_has_trailing_icon > div > span {{
            font-size: 1.5em !important;  
        }}
        "#);
        style_str.push_str(has_trailing_icon_str.as_str());
    } 

    let float_label;
    if !no_asterisk {
        float_label = format!(r#"
        input#{id}:focus + label,
        input#{id}:not(:placeholder-shown) + label {{
            padding: 0 0.25em;
            line-height: 0.75em;
            top: {label_on_focus_top};
            left: {label_on_focus_left};
            font-size: 0.75em;
            color: {g_input_border_focus_color};
        }}

        input#{id}:not(:focus) + label {{
            color: {g_label_text_color};
        }}
        "#);
    } else {
        float_label = format!(r#"
        input#{id}:focus + label,
        input#{id}:not(:placeholder-shown) + label {{
            display: none;
        }}
        "#);
    }
    style_str.push_str(float_label.as_str());
    if let Some(switcher) = no_spinner {
        if switcher {
            let no_spinner_str = format!(r#"
            input#{id}::-webkit-outer-spin-button,
            input#{id}::-webkit-inner-spin-button {{
                -webkit-appearance: none;
                margin: 0; 
            }}
            
            input#{id}[type=number] {{
                -moz-appearance:textfield;
            }}
            "#);
            style_str.push_str(no_spinner_str.as_str());
        }
    }
    Style::new(style_str).expect("Failed to create style for input field")
}

