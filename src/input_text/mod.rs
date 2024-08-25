//! # GTextInput
//! is similar to google material text field (but not identical) `https://material-web.dev/components/text-field`
//! It allows you to choose style, add leading and/or trailing icons, or leading and/or trailing icon buttons. 
//! 
//! The key size attribute of input field is `font_size`. It bonds a lot of other sizes of input text field and has the default value 16px. 
//! According to this 1px here = 0.0625em
//! 
//! GTextInput has a lot of attributes, but only `id`, onchange and `label` are required. Label here has the same role as placeholder. If you do not need `label`, add it with empty double quotes `""`.
//! All other attributes with default parameters:
//! - style: GInputStyle,
//! [default GInputStyle::Outlined]
//! - name: `AttrValue`,
//! [default ""]
//! - event: `GInputEvent`,
//! [default GInputEvent::OnChange]
//! - input_type: `AttrValue`, 
//! [default "text"]. Another variants: password, search, number, month, url, etc.
//! - class: `AttrValue`,
//! [default ""]
//! - required: `bool`,
//! [default false]
//! - multiple: `bool`,
//! [default false]
//! - readonly: `bool`,
//! [default false]
//! - maxlength: `Option<i32>`,
//! [default None]
//! - max: `Option<AttrValue>`,
//! [default None]
//! - minlength: `Option<i32>,`
//! [default None]
//! - min: `Option<AttrValue>`,
//! [default None]
//! - step: `Option<AttrValue>`, 
//! [default None]
//! - no_spinner: `Option<bool>`, 
//! [default None]
//! - value: `AttrValue`, 
//! [default ""]
//! - autocomplete: `AttrValue`, 
//! [default "off"]
//! - autofocus: bool,
//! [default false]
//! - disabled: bool,
//! [default false]
//! - inputmode: `Option<AttrValue>`,
//! [default None]
//! - id: `AttrValue`,
//! - label: `AttrValue`,
//! - onchange: `Callback<AttrValue>`,
//! - width: `AttrValue`, 
//! [default "100%"]
//! - height: `Option<AttrValue>`
//! [default 3.5em] Be careful to change this! It can break the sizes of text field. Better use `em`, e.g. `3.4em` or `2em`
//! - font_size: `AttrValue`, 
//! [default "16px"]
//! - border_radius: `AttrValue`, 
//! [default "4px"] It is similar to container_shape in google material buttons
//! - border_color: `AttrValue`, 
//! [default "grey"]
//! - border_color_hover: `AttrValue`, 
//! [default "black"]
//! - border_focus_color: `AttrValue`, 
//! [default "#6200ee"]
//! - label_background_color: `AttrValue`, 
//! [default "white"]
//! - label_text_color: `AttrValue`, 
//! [default "#aaa"]
//! - align_supporting_text: `AttrValue`, 
//! [default "left"]
//! - supporting_text_color: `Option<AttrValue>`, 
//! [default None] e.g. `black` or `red` or `#ffffff`
//! - supporting_text: `Option<AttrValue>`, 
//! [default None] e.g. `*required` or `Error`
//! - no_asterisk: `bool`, 
//! [default false]
//! - has_leading_icon: `bool`, 
//! [default false]
//! - has_trailing_icon: `bool`, 
//! [default false]
//! 
//! See the describtion of this attributes here: `https://material-web.dev/components/text-field/#api`
//! 
//! ## Examples
//! ```
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! let onchange_username = Callback::from(|username: AttrValue| {Msg::InputUsername(username)});
//! 
//! <GTextInput
//! id="username_text_login_name"
//! onchange={onchange_username} 
//! label="Имя пользователя" />
//! ```
//! 
//! Also you can add leading and trailing GIcons, change style, change Event to InputEvent and do many other things via attributes in this way:
//! 
//! ```
//! use yew::prelude::*;
//! use yew_google_material::prelude::*;
//! 
//! let search_input = Callback::from(|search_input: AttrValue| {Msg::Search(search_input)});
//! 
//! <GTextInput
//!     id="username_text_login_name"
//!     event={GInputEvent::OnInput}
//!     oninput={search_input} 
//!     has_leading_icon=true
//!     has_trailing_icon=true
//!     input_type="text" 
//!     supporting_text="text"
//!     label="Введите поисковый запрос" >
//!     <GIcon 
//!        icon="search" 
//!        leading_icon=true
//!        icon_style={GIconStyle::Outlined} 
//!     />
//!     <GIcon 
//!        icon="cancel" 
//!        trailing_icon=true
//!        icon_style={GIconStyle::Outlined} 
//!     />
//! </GTextInput>
//! ```
//! 
//! If you need to add trailing button icon inside input field, instead of `GIcon` use `GButton` inside `<GTextInput></GTextInput>` with attributes:
//! `has_icon` (icon name from `fonts.google.com/icons`), `trailing_icon` (`true`), `parent` (`DependsOn::GTextInput`), `icon_style` (Outlined, Rounded or Sharp)
//! 
//! Do not use `label` attribute for `GButton` inside `GTextInput`!
//! ```
//! <GTextInput
//!     id="username_text_login_name"
//!     onchange={username_input} 
//!     input_type="text" 
//!     has_trailing_icon=true
//!     supporting_text="text"
//!     label="Введите поисковый запрос" >
//!     <GButton 
//!         id="login_button"
//!         button_type="button"
//!         parent={DependsOn::GTextInput}      // required inside GTextInput
//!         style={GButtonStyle::Outlined}      // required for icon inside GButton
//!         label_color="#6750A4"
//!         has_icon="login"                    // required for icon inside GButton
//!         trailing_icon=true
//!         icon_style={GIconStyle::Outlined}   // required for icon inside GButton
//!     />
//! </GTextInput>
//! ```
//! If you need leading button icon element inside `GTextInput`, just remove `trailing_icon` attribute from `GButton`, add `has_leading_icon=true` for `GTextInput` and remove `has_trailing_icon=true`. 
//! Attention! It is recomended to use `button_type` attribute with `"button"`, or your button will be on its own inside `<form></form>` element.

pub(crate) mod input_text_css;
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};

use crate::{input_text::input_text_css::input_style, GInputStyle};

#[derive(Debug, Clone)]
pub enum Msg {
    InputTextInit,
    InputTextOnchange,
    InputTextOninput,
}

#[derive(PartialEq, Default)]
pub enum GInputEvent {
    #[default]
    OnChange,
    OnInput,
}

#[derive(Properties, PartialEq)]
pub struct GTextInputProps {
    #[prop_or_default]
    pub style: GInputStyle,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub event: GInputEvent,
    #[prop_or_else(|| AttrValue::from("text"))]
    pub input_type: AttrValue, 
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub maxlength: Option<i32>,
    #[prop_or_default]
    pub max: Option<AttrValue>,
    #[prop_or_default]
    pub minlength: Option<i32>,
    #[prop_or_default]
    pub min: Option<AttrValue>,
    #[prop_or_default]
    pub step: Option<AttrValue>, 
    #[prop_or_default]
    pub no_spinner: Option<bool>, 
    #[prop_or_default]
    pub value: AttrValue, 
    #[prop_or_else(|| AttrValue::from("off"))]
    pub autocomplete: AttrValue, 
    #[prop_or_default]
    pub pattern: Option<AttrValue>, 
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inputmode: Option<AttrValue>,
    pub id: AttrValue,
    pub label: AttrValue,
    pub onchange: Callback<AttrValue>,
    #[prop_or_else(|| AttrValue::from("100%"))]
    pub width: AttrValue, 
    #[prop_or_default]
    pub height: Option<AttrValue>, 
    #[prop_or_else(|| AttrValue::from("16px"))]
    pub font_size: AttrValue, 
    #[prop_or_else(|| AttrValue::from("4px"))]
    pub border_radius: AttrValue, 
    #[prop_or_else(|| AttrValue::from("grey"))]
    pub border_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("black"))]
    pub border_color_hover: AttrValue, 
    #[prop_or_else(|| AttrValue::from("#6200ee"))]
    pub border_focus_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("white"))]
    pub label_background_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("#aaa"))]
    pub label_text_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("left"))]
    pub align_supporting_text: AttrValue, 
    #[prop_or_default]
    pub supporting_text_color: Option<AttrValue>, 
    #[prop_or_default]
    pub supporting_text: Option<AttrValue>, 
    #[prop_or_default]
    pub no_asterisk: bool, 
    #[prop_or_default]
    pub has_leading_icon: bool, 
    #[prop_or_default]
    pub has_trailing_icon: bool, 
    #[prop_or_default]
    pub children: Html,
}

pub struct GTextInput {
    refs: NodeRef,
}

impl Component for GTextInput {
    type Message = Msg;
    type Properties = GTextInputProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        assert!(!ctx.props().id.is_empty());
        Self {
            refs: NodeRef::default(),
        }
    } 

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputTextInit => {
                let input = self.refs.cast::<HtmlInputElement>().unwrap();
                if let Some(maxlength) = ctx.props().maxlength {
                    input.set_max_length(maxlength)
                }
                if let Some(max) = &ctx.props().max {
                    input.set_max(max)
                }
                if let Some(minlength) = ctx.props().minlength {
                    input.set_min_length(minlength)
                }
                if let Some(min) = &ctx.props().min {
                    input.set_min(min)
                }
                if let Some(inputmode) = &ctx.props().inputmode {
                    input.set_input_mode(inputmode)
                }
                if let Some(step) = &ctx.props().step {
                    input.set_step(step)
                }
                if let Some(pattern) = &ctx.props().pattern {
                    input.set_step(pattern)
                }
            },
            Msg::InputTextOnchange => {
                let input = self.refs.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    ctx.props().onchange.emit(AttrValue::from(input.value()));
                } 
            },
            Msg::InputTextOninput => {
                let input = self.refs.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    ctx.props().onchange.emit(AttrValue::from(input.value()));
                } 
            },
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let g_init = AttrValue::from(format!("g_init_{}", ctx.props().id));
        let g_container = AttrValue::from(format!("g_container_{}", ctx.props().id));
        let stylesheet = input_style(
            &ctx.props().style,
            ctx.props().id.clone(),
            g_init.clone(),
            g_container.clone(),
            ctx.props().width.clone(), 
            ctx.props().height.clone(), 
            ctx.props().font_size.clone(), 
            ctx.props().border_radius.clone(), 
            ctx.props().border_color.clone(), 
            ctx.props().border_color_hover.clone(), 
            ctx.props().border_focus_color.clone(), 
            ctx.props().label_background_color.clone(), 
            ctx.props().label_text_color.clone(), 
            ctx.props().align_supporting_text.clone(), 
            ctx.props().supporting_text_color.clone(),
            ctx.props().no_asterisk.clone(), 
            ctx.props().has_leading_icon.clone(), 
            ctx.props().has_trailing_icon.clone(), 
            ctx.props().no_spinner.clone(), 
        );
        let onfocus = ctx.link().callback(|_| Msg::InputTextInit);
        let onchange = ctx.link().callback(|_| Msg::InputTextOnchange);
        let oninput = ctx.link().callback(|_| Msg::InputTextOninput);
        let input_event: VNode = match ctx.props().event {
            GInputEvent::OnChange => html! {
                <input 
                    ref={&self.refs}
                    {onfocus}
                    {onchange} 
                    class={&ctx.props().class} 
                    id={&ctx.props().id}
                    type={&ctx.props().input_type} 
                    name={&ctx.props().name} 
                    value={&ctx.props().value}
                    required={ctx.props().required} 
                    autofocus={ctx.props().autofocus}
                    autocomplete={&ctx.props().autocomplete}
                    multiple={ctx.props().multiple}
                    readonly={ctx.props().readonly}
                    disabled={ctx.props().disabled}
                    placeholder={""}
                />
            },
            GInputEvent::OnInput => html! {
                <input 
                    ref={&self.refs}
                    {onfocus}
                    {oninput} 
                    class={&ctx.props().class} 
                    id={&ctx.props().id}
                    type={&ctx.props().input_type} 
                    name={&ctx.props().name} 
                    value={&ctx.props().value}
                    required={ctx.props().required} 
                    autofocus={ctx.props().autofocus}
                    autocomplete={&ctx.props().autocomplete}
                    multiple={ctx.props().multiple}
                    readonly={ctx.props().readonly}
                    disabled={ctx.props().disabled}
                    placeholder={""}
                />
            }
        };
        html! {
            <stl class={stylesheet}>
                <div id={g_init}>  
                    <div id={g_container}>  
                        {input_event}
                        <label for={&ctx.props().id}>{&ctx.props().label}</label>
                        {ctx.props().children.clone()}
                    </div>
                    <div class="g_supporting_text_below_input_text_field">
                        {ctx.props().supporting_text.clone()}
                    </div>
                </div>
            </stl>
        }
    }
}