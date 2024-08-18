//! # GButton
//! is similar to google material common buttons (not identical)
//! 
//! The key size attribute of input field is `font_size`. It bonds a lot of other sizes of text field and has the default value 14px. 
//! According to this 1px here = 0.0714em
//! 
//! GTextInput has a lot of attributes (and you can make something similar to FAB button via them), but only id are required.
//! 
//! Attention! You must set `label` and/or use icon to make your button readable! 
//! 
//! All other attributes with default parameters:
//!- label: `AttrValue`,
//![default ""] Attention! If you do not use icon in button, you must use this!
//!- button_type: `AttrValue`,
//![default "button"]
//!- style: `GButtonStyle`,
//![default GButtonStyle::Filled]
//!- outlined_border_color: `Option<AttrValue>`,
//![default "#79747E"]
//!- font_size: `AttrValue`, 
//![default "14px"]
//!- onclick: `Option<Callback<PointerEvent>>`,
//![default None] Use PointerEvent instead of MouseEvent
//!- class: `AttrValue`,
//![default ""]
//!- height: `AttrValue`,
//![default "2.85em"]
//!- width: `Option<AttrValue>`,
//![default None]
//!- background_color: `AttrValue`,
//![default "#6750A4"]
//!- text_color: `AttrValue`, 
//![default "#000"]
//!- border_radius: `AttrValue`,
//![default "20px"] It is similar to container_shape in google material buttons
//!- has_icon: `bool`,
//![default false]
//!- trailing_icon: `bool`,
//![default false]
//!- dark_theame: `bool`,
//![default false] Experimental! Now it changes shadows from black to white if true.
//!- disabled: `bool`,
//![default false]
//! 
//! ## Examples
//! ```
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! GButton 
//! id="use_g_button" 
//! label="Button" />
//! ```
//! 
//! Also you can add icon with `has_icon` attribute. If you need trailing icon use both `has_icon` and `trailing_icon` with `true`
//! 
//! ```
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! <GButton 
//! id="login_button"
//! label="Sign In"
//! style={GButtonStyle::Outlined}
//! button_type="submit"
//! text_color="white"
//! has_icon=true
//! >
//! <GIcon 
//!     icon="login" 
//!     leading_icon=true
//!     icon_style={GIconStyle::Outlined} 
//! />
//! </GButton>
//! ```
//! Attention! If you change icon size within button you can break the design. Probably then you need to adjust width and height. Do it with caution.
use button_css::input_style;
use gloo_timers::future::TimeoutFuture;
use web_sys::HtmlElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::GButtonStyle;

mod button_css;

pub enum Msg {
    OnPointerDown(PointerEvent),
    OnKeyPress(KeyboardEvent),
    OnPointerUp(PointerEvent),
}

#[derive(Properties, PartialEq)]
pub struct GButtonProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_else(|| AttrValue::from("button"))]
    pub button_type: AttrValue,
    #[prop_or_default]
    pub style: GButtonStyle,
    #[prop_or_default]
    pub outlined_border_color: Option<AttrValue>,
    #[prop_or_else(|| AttrValue::from("14px"))]
    pub font_size: AttrValue, 
    #[prop_or_default]
    pub onclick: Option<Callback<PointerEvent>>,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_else(|| AttrValue::from("2.85em"))]
    pub height: AttrValue,
    #[prop_or_default]
    pub width: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_else(|| AttrValue::from("#6750A4"))]
    pub background_color: AttrValue,
    #[prop_or_else(|| AttrValue::from("#FFFFFF"))]
    pub label_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("20px"))]
    pub border_radius: AttrValue,
    #[prop_or_default]
    pub has_icon: bool,
    #[prop_or_default]
    pub trailing_icon: bool,
    #[prop_or_default]
    pub dark_theame: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub autofocus: bool,
}

pub struct GButton {
    button: NodeRef,
    only_icon: bool,
    pointer_id: Option<i32>,
}

impl Component for GButton {
    type Message = Msg;

    type Properties = GButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let only_icon: bool = if ctx.props().label == AttrValue::default() {true} else {false};
        Self {
            button: NodeRef::default(),
            only_icon,
            pointer_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnPointerDown(event) => {
                let onmouse = true;
                let button = self.button.cast::<HtmlElement>().unwrap();
                self.pointer_id = Some(event.pointer_id());
                button.set_pointer_capture(self.pointer_id.unwrap()).unwrap();
                let button_rect = button.get_bounding_client_rect();
                let x = {
                    let x = event.client_x() - button_rect.left().round() as i32;
                    format!("{x}px")
                };
                let y = {
                    let y = event.client_y() - button_rect.top().round() as i32;
                    format!("{y}px")
                };
                if !ctx.props().disabled {
                    ripple_effect(onmouse, &x, &y, button, &ctx.props().id);
                }
            },
            Msg::OnKeyPress(event) => {
                if event.key() == "Enter" {
                    let onmouse = false;
                    let button = self.button.cast::<HtmlElement>().unwrap();
                    let x = {
                        let x = button.offset_width() / 2;
                        format!("{x}px")
                    };
                    let y = {
                        let y = button.offset_height() / 2;
                        format!("{y}px")
                    };
                    if !ctx.props().disabled {
                        ripple_effect(onmouse, &x, &y, button, &ctx.props().id);
                    }
                    if let Some(onclick) = ctx.props().onclick.as_ref() {
                        onclick.emit(PointerEvent::new("pointerup").expect("Key to Pointer fail"));
                    }
                }
            },
            Msg::OnPointerUp(event) => {
                if self.pointer_id.is_some() {
                    let g_span_ripple_selector = AttrValue::from(format!("span#g_init_span{}", ctx.props().id));
                    if let Some(span) = self.button.cast::<HtmlElement>().unwrap().query_selector(&g_span_ripple_selector).unwrap() {
                        span.remove()
                    }
                    self.button.cast::<HtmlElement>().unwrap().release_pointer_capture(self.pointer_id.expect("No button pointer id")).unwrap();
                    if let Some(value) = ctx.props().onclick.as_ref() {
                        value.emit(event)
                    }
                    self.pointer_id = None;
                }
            },
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let g_init = AttrValue::from(format!("g_init_{}", ctx.props().id));
        let stylesheet = input_style(
            &ctx.props().style,
            &ctx.props().id,
            self.only_icon,
            &g_init,
            &ctx.props().font_size,
            &ctx.props().height,
            &ctx.props().width,
            &ctx.props().background_color,
            ctx.props().label_color.clone(),
            &ctx.props().outlined_border_color,
            &ctx.props().border_radius,
            ctx.props().disabled,
            ctx.props().has_icon,
            ctx.props().trailing_icon,
            ctx.props().dark_theame,
        );

        let onpointerdown = ctx.link().callback(|event: PointerEvent| Msg::OnPointerDown(event));
        let onkeydown = ctx.link().callback(|event: KeyboardEvent| Msg::OnKeyPress(event));
        let onpointerup = ctx.link().callback(|event: PointerEvent| Msg::OnPointerUp(event));
        html! {
            <stl class={stylesheet}>
                <div id={g_init}>
                    <button 
                        id={ctx.props().id.clone()} 
                        type={ctx.props().button_type.clone()}
                        ref={&self.button}
                        class={&ctx.props().class}
                        {onpointerdown}
                        {onkeydown}
                        {onpointerup}
                        aria-label={ctx.props().id.clone()} 
                        disabled={ctx.props().disabled}
                        autofocus={ctx.props().autofocus}
                    >
                        {&ctx.props().label}
                    </button>
                    {ctx.props().children.clone()}
                </div>
            </stl>
        }
    }
}

fn ripple_effect(onmouse: bool, x: &str, y: &str, button: HtmlElement, id: &AttrValue) {
    let span = button
        .owner_document()
        .unwrap()
        .create_element("span")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let g_span_ripple = AttrValue::from(format!("g_init_span{}", id));
    span.set_id(&g_span_ripple);
    span.style().set_property("left", &x).unwrap();
    span.style().set_property("top", &y).unwrap();
    
    button.append_child(&span).unwrap();
    if !onmouse {
        spawn_local(async move {
            TimeoutFuture::new(300).await;
            span.remove()
        })
    }
}