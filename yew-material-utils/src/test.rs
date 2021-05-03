use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew::utils::document;
use yew::virtual_dom::VNode;

use crate::loader::{link, script};
use crate::node::{YNode, YProps};

#[wasm_bindgen(module = "/src/test.js")]
extern "C" {
    fn init();
}

pub async fn run(name: &str, node: VNode) -> (String, Element) {
    let div = document().create_element("div").unwrap();
    div.set_attribute("id", &name).unwrap();
    div.set_attribute("style", "margin: 10px 0;background: #c3c3c3;")
        .unwrap();
    let body = document().query_selector("body").unwrap().unwrap();
    body.append_child(&div).unwrap();

    let output = document().get_element_by_id("output").unwrap();
    if let Some(_) = output.get_attribute("style") {
        //
    } else {
        body.set_attribute("style", "background: #efefef").unwrap();
        output.set_attribute("style", "background: #000;color: #fff;padding: 20px 20px 10px 20px;line-height: 12px;font-size: 12px;white-space: pre-wrap;").unwrap();
        script("https://unpkg.com/jss/dist/jss.min.js", false).await;
        script(
            "https://unpkg.com/jss-preset-default/dist/jss-preset-default.min.js",
            false,
        )
        .await;
        script("https://unpkg.com/@material/mwc-button", true).await;
        link("https://fonts.googleapis.com/css?family=Roboto:300,400,500").await;
        link("https://fonts.googleapis.com/css?family=Material+Icons&display=block").await;
        init();
    }
    App::<YNode>::new().mount_with_props(
        document().get_element_by_id(&name).unwrap(),
        YProps { node: node.clone() },
    );

    let doc = document().get_element_by_id(&name).unwrap();
    let html = doc.inner_html();

    (html, doc)
}
