use stylist::Style;
use yew::prelude::*;
use crate::IconStyle;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct IconProps {
    #[prop_or_default]
    pub icon: AttrValue,
    pub icon_style: IconStyle,
    #[prop_or_else(|| false )]
    pub fill: bool,
    #[prop_or_else(|| AttrValue::from("200"))]
    pub wght: AttrValue,
    #[prop_or_else(|| AttrValue::from("0"))]
    pub grade: AttrValue,
    #[prop_or_else(|| AttrValue::from("24"))]
    pub opsz: AttrValue,
    #[prop_or_else(|| AttrValue::from(r#"#000"#))]
    pub color: AttrValue,
    #[prop_or_else(|| AttrValue::from("100%"))]
    pub width: AttrValue,
    #[prop_or_else(|| AttrValue::from("100%"))]
    pub height: AttrValue,
    #[prop_or_else(|| AttrValue::from("unset"))]
    pub transition: AttrValue,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let universal_slyle = match props.icon_style {
        IconStyle::Outlined => "material-symbols-outlined",
        IconStyle::Rounded => "material-symbols-rounded",
        IconStyle::Sharp => "material-symbols-sharp",
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
    let width = props.width.clone();
    let height = props.height.clone();
    let transition = props.transition.clone();

    let style_str = format!(
        r#"
            .{universal_slyle} {{
                transition: {transition};
                color: {color};
                width: {width};
                height: {height};
                font-variation-settings:
                'FILL' {fill},
                'wght' {wght},
                'GRAD' {grade},
                'opsz' {opsz}
                }}
        "#
    );

let style = Style::new(style_str).expect("Failed to create style");
    html! {
        <stl class={style}>
            <span class={universal_slyle}>{props.icon.clone()}</span>
        </stl>
    }
}