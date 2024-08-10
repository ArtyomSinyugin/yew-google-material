//! # GTextInput
//! is similar to google material text field (but not identical) `https://material-web.dev/components/text-field`
//! 
//! The key size attribute of input field is `g_font_size`. It bonds a lot of other sizes of text field and has the default value 16px. 
//! 
//! GTextInput has a lot of attributes, but only id, onchange and label are required. Label here has the same role as placeholder. If you do not need label, add it with empty double quotes.
//! All other attributes with default parameters:
//! - input_style: GInputStyle,
//! [default GInputStyle::Outlined]
//! - name: `AttrValue`,
//! [default ""]
//! - event: `GInputEvent`,
//! [default GInputEvent::OnChange]
//! - input_type: `AttrValue`, 
//! [default "text"]. Another variants: password, search, number, month, url, etc.
//! - input_class: `AttrValue`,
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
//! - g_input_width: `AttrValue`, 
//! [default "100%"]
//! - g_font_size: `AttrValue`, 
//! [default "16px"]
//! - g_input_border_radius: `AttrValue`, 
//! [default "4px"]
//! - g_input_border_color: `AttrValue`, 
//! [default "grey"]
//! - g_input_border_color_hover: `AttrValue`, 
//! [default "black"]
//! - g_input_border_focus_color: `AttrValue`, 
//! [default "#6200ee"]
//! - g_label_background_color: `AttrValue`, 
//! [default "white"]
//! - g_label_text_color: `AttrValue`, 
//! [default "#aaa"]
//! - g_align_supporting_text: `AttrValue`, 
//! [default "left"]
//! - g_supporting_text_color: `Option<AttrValue>`, 
//! [default None]
//! - supporting_text: `Option<AttrValue>`, 
//! [default None]
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
//! label="Имя пользователя" >
//! </GTextInput>
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
    pub input_style: GInputStyle,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub event: GInputEvent,
    #[prop_or_else(|| AttrValue::from("text"))]
    pub input_type: AttrValue, 
    #[prop_or_default]
    pub input_class: AttrValue,
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
    pub g_input_width: AttrValue, 
    #[prop_or_else(|| AttrValue::from("16px"))]
    pub g_font_size: AttrValue, 
    #[prop_or_else(|| AttrValue::from("4px"))]
    pub g_input_border_radius: AttrValue, 
    #[prop_or_else(|| AttrValue::from("grey"))]
    pub g_input_border_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("black"))]
    pub g_input_border_color_hover: AttrValue, 
    #[prop_or_else(|| AttrValue::from("#6200ee"))]
    pub g_input_border_focus_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("white"))]
    pub g_label_background_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("#aaa"))]
    pub g_label_text_color: AttrValue, 
    #[prop_or_else(|| AttrValue::from("left"))]
    pub g_align_supporting_text: AttrValue, 
    #[prop_or_default]
    pub g_supporting_text_color: Option<AttrValue>, 
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
            &ctx.props().input_style,
            ctx.props().id.clone(),
            g_init.clone(),
            g_container.clone(),
            ctx.props().g_input_width.clone(), 
            ctx.props().g_font_size.clone(), 
            ctx.props().g_input_border_radius.clone(), 
            ctx.props().g_input_border_color.clone(), 
            ctx.props().g_input_border_color_hover.clone(), 
            ctx.props().g_input_border_focus_color.clone(), 
            ctx.props().g_label_background_color.clone(), 
            ctx.props().g_label_text_color.clone(), 
            ctx.props().g_align_supporting_text.clone(), 
            ctx.props().g_supporting_text_color.clone(),
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
                    class={&ctx.props().input_class} 
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
                    class={&ctx.props().input_class} 
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