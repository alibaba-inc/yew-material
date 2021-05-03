use crate::{add_listener, dispatch, remove_listener, IframeService, StringFeatures};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use spin::Mutex;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/theme.js")]
extern "C" {
    fn theme_ident_pre() -> String;
    fn theme(ident: &str, is_iframe: bool) -> JsValue;
    fn set_theme(ident: &str, theme: &JsValue);
    fn change_theme_to_window(callback: JsValue);
    fn change_theme_to_iframe(ident: &str, event: &str);
}

lazy_static! {
    static ref THEME_IDENT: Mutex<ThemeIdent> = Mutex::new(ThemeIdent {
        name: "".into(),
        tag: "__mdc_theme_ident",
        event: "on_change_theme",
    });
}

#[derive(Deserialize, Serialize)]
pub struct Theme<T> {
    pub ident: String,
    pub font_family: String,
    pub font_weight: u32,
    pub color: String,
    pub background: String,
    pub ripple: String,
    pub activated: String,
    pub selected: String,
    pub hover: String,
    pub disabled: String,
    pub body: ThemeCB,
    pub divider: ThemeC,
    pub skeleton: ThemeB,
    pub button: ThemeButton,
    pub dialog: ThemeDialog,
    pub scrollbar: ThemeB,
    pub icon: ThemeC,
    pub appbar: ThemeAppbar,
    pub list: ThemeList,
    pub list_item: ThemeListItem,
    pub tab_item: ThemeTabItem,
    pub radio: ThemeCkUnck,
    pub checkbox: ThemeCkUnck,
    pub switch: ThemeCkUnck,
    pub slider: ThemeCB,
    pub progress: ThemeProgress,
    pub textfield: ThemeTextfield,
    pub snackbar: ThemeCB,
    pub extra: T,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeC {
    pub color: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeB {
    pub background: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeCB {
    pub color: String,
    pub background: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeAppbar {
    pub shadow: String,
    pub background: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeButton {
    pub color: String,
    pub text_button_color: String,
    pub background: String,
    pub disabled_color: String,
    pub disabled_background: String,
    pub ripple: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeDialog {
    pub heading_color: String,
    pub content_color: String,
    pub background: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeCkUnck {
    pub checked: String,
    pub unchecked: String,
    pub disabled: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeList {
    pub background: String,
    pub ripple: String,
    pub selected: String,
    pub hover: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeListItem {
    pub secondary_color: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeTabItem {
    pub color: String,
    pub icon_color: String,
    pub activated: String,
    pub ripple: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeProgress {
    pub color: String,
    pub linear_buffer: String,
    pub linear_buffer_dots: String,
}

#[derive(Deserialize, Serialize)]
pub struct ThemeTextfield {
    pub color: String,
    pub background: String,
    pub error_color: String,
    pub label_color: String,
    pub label_focus_color: String,
    pub icon_color: String,
    pub disabled_color: String,
    pub disabled_background: String,
    pub radius: String,
    pub border_color: String,
    pub border_hover_color: String,
    pub border_disabled_color: String,
}

struct ThemeIdent {
    name: String,
    tag: &'static str,
    event: &'static str,
}

impl ThemeIdent {
    fn set(&mut self, ident: String) {
        self.name = ident;
    }
}

pub fn init<E>(ident: String, theme: E)
where
    E: DeserializeOwned + Serialize + ?Sized,
{
    set_theme(&ident, &JsValue::from_serde(&theme).unwrap());
    set_theme_ident_local(&ident);
    THEME_IDENT.lock().set(ident);
    change_theme_to_js(Box::new(|ident| {
        change_theme(ident);
    }));
}

pub(crate) fn change_theme_to_js(callback: Box<dyn Fn(String)>) {
    let cb = Closure::wrap(callback);
    change_theme_to_window(cb.as_ref().clone());
    cb.forget();
}

pub fn get_theme_ident(is_local: bool) -> String {
    match is_local {
        true => get_theme_ident_local(),
        _ => THEME_IDENT.lock().name.clone(),
    }
}

pub fn change_theme(ident: String) {
    if !IframeService::satisfy() {
        change_theme_to_iframe(&ident, &THEME_IDENT.lock().event);
        dispatch(THEME_IDENT.lock().event);
    }
    set_theme_ident_local(&ident);
    THEME_IDENT.lock().set(ident);
}

pub fn set_theme_ident_local(ident: &str) {
    String::set_storage(THEME_IDENT.lock().tag, ident);
}

pub fn get_theme_ident_local() -> String {
    String::get_storage(THEME_IDENT.lock().tag)
}

// #[deprecated]
// pub fn get_theme_ident_pre() -> &'static str {
//     Box::leak(theme_ident_pre().into_boxed_str())
// }

pub fn to_theme<T>() -> Theme<T>
where
    T: DeserializeOwned + Serialize,
{
    theme(&THEME_IDENT.lock().name, IframeService::satisfy())
        .into_serde::<Theme<T>>()
        .unwrap()
}

pub fn on_change(uuid: &str, callback: Box<dyn Fn()>) {
    add_listener(uuid, THEME_IDENT.lock().event, callback);
}

pub enum Msg {
    Reload,
}

pub enum DataMsg<T> {
    Reload,
    Callback(Box<dyn Fn(&mut T) -> bool>),
}

pub fn reload<T>(uuid: &str, link: &ComponentLink<T>)
where
    T: yew::Component,
    Msg: Into<T::Message>,
{
    let self_link = link.clone();
    on_change(uuid, Box::new(move || self_link.send_message(Msg::Reload)));
}

pub fn reload_with_data<T>(uuid: &str, link: &ComponentLink<T>)
where
    T: yew::Component,
    DataMsg<T>: Into<T::Message>,
{
    let self_link = link.clone();
    on_change(
        uuid,
        Box::new(move || self_link.send_message(DataMsg::Reload)),
    );
}

pub fn destroy(uuid: &String) {
    remove_listener(&uuid, THEME_IDENT.lock().event);
}
