use futures_channel::oneshot;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::utils::document;
//use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};

//Material_mwc version
pub static VERSION: &str = "0.21.0";

pub async fn script(url: &str, module: bool) {
    let selector = format!("script[src='{}']", url);
    let node = document().query_selector(&selector);

    let (tx, rx) = oneshot::channel::<u32>();
    let mut tx = Some(tx);

    if let Some(_) = node.unwrap() {
        drop(tx.take().unwrap());
    } else {
        let ele = document().create_element("script").unwrap();
        if module {
            ele.set_attribute("type", "module").unwrap();
            ele.set_attribute("src", &format!("{}@{}?module", url, VERSION))
                .unwrap();
        } else {
            ele.set_attribute("src", url).unwrap();
        }
        document()
            .query_selector("head")
            .unwrap()
            .unwrap()
            .append_child(&ele)
            .unwrap();

        let closure = Closure::wrap(Box::new(move |_: JsValue| {
            drop(tx.take().unwrap());
        }) as Box<dyn FnMut(_)>);

        ele.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    rx.await.unwrap_err();
}

pub async fn link(url: &str) {
    let selector = format!("link[href='{}']", url);
    let node = document().query_selector(&selector);

    let (tx, rx) = oneshot::channel::<u32>();
    let mut tx = Some(tx);

    if let Some(_) = node.unwrap() {
        drop(tx.take().unwrap());
    } else {
        let ele = document().create_element("link").unwrap();
        ele.set_attribute("href", url).unwrap();
        ele.set_attribute("rel", "stylesheet").unwrap();
        document()
            .query_selector("head")
            .unwrap()
            .unwrap()
            .append_child(&ele)
            .unwrap();

        let closure = Closure::wrap(Box::new(move |_: JsValue| {
            drop(tx.take().unwrap());
        }) as Box<dyn FnMut(_)>);

        ele.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    rx.await.unwrap_err();
}
