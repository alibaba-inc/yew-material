#[macro_use]
extern crate lazy_static;

use anyhow::Error;
use gloo_timers::future::TimeoutFuture;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, HtmlElement, ShadowRoot, Storage};
use yew::{
    format::{Json, Nothing, Text},
    utils::{document, window},
    Callback, Html,
};
use yew_services::fetch::{FetchService, FetchTask, Request, Response};

pub mod theme;
pub use self::theme::*;

pub mod style;
pub use self::style::*;

pub mod loader;
pub use self::loader::*;

pub mod plugins;
pub use self::plugins::*;

pub mod test;
pub use self::test::*;

pub mod node;
pub use self::node::*;

#[wasm_bindgen(module = "/src/lib.js")]
extern "C" {
    pub static PUBLIC_URL: String;
    pub static InnerWidth: f64;
    pub fn imports(path: &str);
    pub fn event(message: &str) -> Event;
    pub fn bind_listener(uuid: &str, r#type: &str, callback: JsValue);
    pub fn remove_listener(uuid: &str, r#type: &str);
    pub fn bind_element_listener(uuid: &str, element: &Element, r#type: &str, callback: JsValue);
    pub fn remove_element_listener(uuid: &str, element: &Element, r#type: &str);
    pub fn bind(uuid: &str, element: &Element, selector: &str, r#type: &str, callback: JsValue);
    pub fn off(uuid: &str, element: &Element, r#type: &str);
    pub fn find_element(element: &Element, selector: &str) -> Element;
    pub fn set_element_attr(element: &Element, attr: &str, value: &Element);
    pub fn remove_children_attr(element: &Element, selector: &str, attr: &str);
    pub fn get_query_string(name: &str) -> String;
    pub fn form_traversing(id: &str) -> JsValue;
    pub fn formValidtrans(id: &str, callback: JsValue);
    pub fn del_form_validtrans(id: &str);
    pub fn anchor(element: &Element, anchor_id: &str);
    pub fn iframe_open();
    pub fn iframe_close();
}

pub mod log {
    pub use log::*;
    pub fn initialize() {
        wasm_logger::init(wasm_logger::Config::default());
    }
}

pub enum CallbackRes<IN, RE> {
    Callback(Rc<dyn Fn(IN) -> RE>),
}

impl<IN, RE, F: Fn(IN) -> RE + 'static> From<F> for CallbackRes<IN, RE> {
    fn from(func: F) -> Self {
        CallbackRes::Callback(Rc::new(func))
    }
}

impl<IN, RE> Clone for CallbackRes<IN, RE> {
    fn clone(&self) -> Self {
        match self {
            CallbackRes::Callback(cb) => CallbackRes::Callback(cb.clone()),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, RE> PartialEq for CallbackRes<IN, RE> {
    fn eq(&self, other: &CallbackRes<IN, RE>) -> bool {
        match (&self, &other) {
            (CallbackRes::Callback(cb), CallbackRes::Callback(other_cb)) => {
                Rc::ptr_eq(cb, other_cb)
            }
        }
    }
}

impl<IN, RE> fmt::Debug for CallbackRes<IN, RE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            CallbackRes::Callback(_) => "CallbackRes<_>",
        };
        f.write_str(data)
    }
}

impl<IN, RE> CallbackRes<IN, RE> {
    pub fn emit(&self, value: IN) -> RE {
        match self {
            CallbackRes::Callback(cb) => cb(value),
        }
    }
}

impl<IN> Default for CallbackRes<IN, bool> {
    fn default() -> CallbackRes<IN, bool> {
        CallbackRes::from(|_| true)
    }
}

pub trait BoolFeatures {
    fn set_storage(name: &str, value: bool);
    fn get_storage(name: &str) -> bool;
}

impl BoolFeatures for bool {
    fn set_storage(name: &str, val: bool) {
        let value = if val { "true" } else { "false" };
        local_storage().set_item(name, value).unwrap();
    }
    fn get_storage(name: &str) -> bool {
        let value = match local_storage().get_item(name).unwrap() {
            Some(i) => i,
            _ => "false".into(),
        };
        match value.as_str() {
            "true" => true,
            _ => false,
        }
    }
}

pub trait StringFeatures {
    fn set_storage(name: &str, value: &str);
    fn get_storage(name: &str) -> String;
}

impl StringFeatures for String {
    fn set_storage(name: &str, value: &str) {
        local_storage().set_item(name, value).unwrap();
    }
    fn get_storage(name: &str) -> String {
        match local_storage().get_item(name).unwrap() {
            Some(i) => i,
            _ => "".into(),
        }
    }
}

pub struct Url {}
impl Url {
    pub fn get_query(name: &str) -> String {
        get_query_string(name)
    }
    pub fn get_path(index: u32) -> String {
        let path = window().location().pathname().unwrap();
        let path_names = to_split(&path, "/");
        let len = (index + 1) as usize;
        let result = match path_names.len() {
            _ if path == "/" => Err("Url::get_path error: url does not have a parameter"),
            x if len > x - 1 => Err("Url::get_path error: index out of bounds"),
            _ => Ok(&path_names[len]),
        };
        let param = result.unwrap();
        let has_dot = to_split(&param, ".");
        match has_dot.len() {
            0 => param.into(),
            _ => has_dot[0].clone(),
        }
    }
}

pub fn local_storage() -> Storage {
    window().local_storage().unwrap().unwrap()
}

pub fn to_split(text: &String, delimiter: &'static str) -> Vec<String> {
    text.split(delimiter).map(|s| s.into()).collect()
}

pub fn set_property(element: &Element, attr: &str, value: &str) {
    element
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .style()
        .set_property(attr, value)
        .unwrap();
}

pub fn dispatch(message: &str) {
    window().dispatch_event(&event(message)).unwrap();
}

pub fn on(
    uuid: &str,
    element: &Element,
    selector: &str,
    r#type: &str,
    callback: Box<dyn Fn(Element)>,
) {
    let cb = Closure::wrap(callback);
    bind(uuid, element, selector, r#type, cb.as_ref().clone());
    cb.forget();
}

pub fn add_listener(uuid: &str, r#type: &str, callback: Box<dyn Fn()>) {
    let cb = Closure::wrap(callback);
    bind_listener(uuid, r#type, cb.as_ref().clone());
    cb.forget();
}

pub fn add_element_listener(
    uuid: &str,
    element: &Element,
    r#type: &str,
    callback: Box<dyn Fn(Element, Event)>,
) {
    let cb = Closure::wrap(callback);
    bind_element_listener(uuid, element, r#type, cb.as_ref().clone());
    cb.forget();
}

pub struct Fetch<'a, T: 'a> {
    _marker: std::marker::PhantomData<&'a T>,
}

type Resp<T> = Response<Json<Result<T, Error>>>;

impl<T> Fetch<'static, T> {
    pub fn new<IN>(request: Request<IN>, callback: Callback<T>) -> Option<FetchTask>
    where
        T: DeserializeOwned + 'static,
        IN: Into<Text>,
    {
        let cb = Callback::from(move |response: Resp<T>| {
            let Json(resp) = response.into_body();
            match resp {
                Ok(data) => {
                    callback.emit(data);
                }
                Err(error) => {
                    log::error!("fetch response error: {}", error);
                }
            }
        });
        Some(FetchService::fetch(request, cb).expect("failed to start request"))
    }
}

pub fn get(api: String) -> Request<Nothing> {
    Request::get(api).body(Nothing).unwrap()
}

pub fn post(api: String, param: Value) -> Request<Result<String, Error>> {
    let text: Text = Json(&param).into();
    Request::post(api)
        .header("Content-Type", "application/json")
        .body(text)
        .unwrap()
}

pub fn element_by_id(id: &str) -> Element {
    document().get_element_by_id(id).unwrap()
}

pub fn get_inner_width() -> f64 {
    window()
        .inner_width()
        .unwrap()
        .as_f64()
        .ok_or("get window innerWidth error!")
        .unwrap()
}

pub fn set_inner_html(html: &str) -> Html {
    let node = document().create_element("div").unwrap();
    node.set_inner_html(html);
    Html::VRef(node.into())
}

pub fn form_validtrans(id: &str, callback: Box<dyn Fn(String) -> bool>) {
    let cb = Closure::wrap(callback);
    formValidtrans(id, cb.as_ref().clone());
    cb.forget();
}

//临时解决方案
//存在隐患，50ms可能还不够
pub struct WebComponents {}
impl WebComponents {
    pub fn try_rendered(element: Element, rendered: Box<dyn Fn(ShadowRoot, Element)>) {
        spawn_local(async move {
            TimeoutFuture::new(50).await;
            rendered(element.shadow_root().unwrap(), element);
        });
    }
    pub fn set_style(id: &str, shadow: &ShadowRoot, style_html: String) {
        match shadow.get_element_by_id(id) {
            Some(style) => {
                style.set_inner_html(&style_html);
            }
            _ => {
                let style = document().create_element("style").unwrap();
                style.set_attribute("id", id).unwrap();
                style.set_inner_html(&style_html);
                shadow.append_child(&style).unwrap();
            }
        };
    }
}

pub struct IframeService {}
impl IframeService {
    pub fn satisfy() -> bool {
        match Url::get_query("_services").as_str() {
            "iframe" => true,
            _ => false,
        }
    }
    pub fn open() {
        if IframeService::satisfy() {
            iframe_open();
        }
    }
    pub fn close() {
        if IframeService::satisfy() {
            iframe_close();
        }
    }
}

pub mod prelude {
    pub use crate::{
        get, log, post,
        style::{create_style, Item, Style},
        theme::{get_theme_ident, init, to_theme, Theme},
        BoolFeatures, Fetch, StringFeatures, Url,
    };
    pub use serde_json::json;
    pub use wasm_bindgen::prelude::*;
    pub use wasm_bindgen_futures::spawn_local;
    pub use yew_services::fetch::{FetchTask, Request};
}

pub use self::prelude::*;
