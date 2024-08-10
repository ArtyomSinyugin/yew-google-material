# Yew Google Material

```toml
yew-google-material = "0.0.4"
```

`yew-google-material` is a very simple crate to use some of google materials and `https://fonts.google.com/`


Unfortunately a lot of crates with materials, icons for yew framework are depricated and use depricated javascript code.

Here I plan to use only Rust code to add some design features for yew.  

Now only text fields and icons are available. Buttons in plans. 

Text fields are not the same as one in google material web, but very similar to them. 

## Icons
To use icons from `https://fonts.google.com/icons` you need to add some html inside 
```html
<head></head>
```

There are three types of icons. Add this if you want to use Outlined style of icons:
```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
```
To use Rounded style of icons:
```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
```
To use Sharp style of icons:
```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Sharp:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
```
Then choose on of the icons in the catalog of https://fonts.google.com/icons, i.e. 'search', 'star', 'menu', etc. and see example below. 

### Examples
```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
```
```rust
use yew::prelude::*;
use yew_google_material::prelude::*;

<GIcon 
    size="16px"
    transition="all 0.2s"
    icon="search" 
    icon_style={GIconStyle::Outlined} 
    fill=true
    wght="200"
    grade="0"
    opsz="24"
    color="#fff"
/>
```

Note, you that you can animate icon attributes with transition. The default value is "unset", but for animation it is recomended to set "all 0.2s" or as you wish.

Or you can add an icon with default options:
```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />
```
```rust
use yew::prelude::*;
use yew_google_material::prelude::*;

<GIcon 
    icon="star" 
    icon_style={GIconStyle::Outlined} 
/>
```

## TextField

See the describtion of this attributes here: `https://material-web.dev/components/text-field/#api`

### Examples
```rust
use yew::prelude::*;
use yew_google_material::prelude::*;

let input_username = Callback::from(|username: AttrValue| {Msg::InputUsername(username)});

<GTextInput
id="username_text_login_name"
onchange={onchange_username} 
label="Имя пользователя" >
</GTextInput>
```

Also you can add leading and trailing GIcons, change style, change Event to InputEvent and do many other things via attributes in this way:

```rust
use yew::prelude::*;
use yew_google_material::prelude::*;

let search_input = Callback::from(|search_input: AttrValue| {Msg::Search(search_input)});

<GTextInput
    id="username_text_login_name"
    event={GInputEvent::OnInput}
    oninput={search_input} 
    has_leading_icon=true
    has_trailing_icon=true
    input_type="text" 
    supporting_text="text"
    label="Введите поисковый запрос" >
    <GIcon 
       icon="search" 
       leading_icon=true
       icon_style={GIconStyle::Outlined} 
    />
    <GIcon 
       icon="cancel" 
       trailing_icon=true
       icon_style={GIconStyle::Outlined} 
    />
</GTextInput>
```

## Crate dependencies
yew = "0.21", features = ["csr"]
stylist = "0.13.0"
web-sys = "0.3.69", features = ["Element"]