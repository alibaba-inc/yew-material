#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
extern crate yew_material_macro;

pub mod example;
pub mod index;
pub mod page;
pub mod route;
pub mod styles;
pub mod theme;

use example::*;
use index::Index;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;
use yew_material_utils::{log, Url};

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    log::initialize();
    theme::initialize();

    let root = document().get_element_by_id("root").unwrap();
    match Url::get_query("exp").as_str() {
        "appbar" => {
            App::<ExpAppbar>::new().mount(root);
        }
        "iframe" => {
            App::<ExpIframe>::new().mount(root);
        }
        _ => {
            App::<Index>::new().mount(root);
        }
    }

    Ok(())
}
