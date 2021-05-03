use crate::theme::{get_theme_ident, to_theme, Theme};
use crate::to_split;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
// use yew::format::{Json, Text};

#[wasm_bindgen(module = "/src/style.js")]
extern "C" {
    fn create_style_sheet(style: &JsValue, meta: &str, json: bool, global: bool) -> String;
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Extra {}

pub struct Style {}

impl Style {
    pub fn easy(value: Value) -> StyleMap {
        create_style::<Extra>(Box::new(move |_| value.to_owned()))
    }
}

pub type StyleMap = HashMap<String, String>;
pub trait Item {
    fn item(&self, key: &str) -> String;
    fn theme_item(&self, key: &str) -> String;
}

impl Item for StyleMap {
    fn item(&self, key: &str) -> String {
        self.get(key)
            .expect(&format!(
                "styleMap parsing error: please check if the key [{}] exists or is misspelled",
                &key
            ))
            .into()
    }
    fn theme_item(&self, key: &str) -> String {
        self.item(key) + " mdc_theme_" + &get_theme_ident(false)
    }
}

pub fn to_styles(sheet: &String) -> StyleMap {
    let sheets: Vec<String> = to_split(&sheet, "|");
    let keys = to_split(&sheets.get(0).unwrap(), ",");
    let vals = to_split(&sheets.get(1).unwrap(), ",");
    let mut styles = HashMap::new();
    let mut num = 0;
    for i in keys {
        styles.insert(i, vals.get(num).unwrap().into());
        num = num + 1;
    }
    styles
}

pub fn create_style<E>(style: Box<dyn Fn(&Theme<E>) -> Value>) -> StyleMap
where
    E: DeserializeOwned + Serialize,
{
    // let text: Text = Json(&style(&to_theme())).into();
    // let sheet = create_style_sheet(&text.unwrap().into(), "customer", true, false);
    let sheet = create_style_sheet(
        &JsValue::from_serde(&style(&to_theme())).unwrap(),
        "customer",
        true,
        false,
    );
    to_styles(&sheet)
}

pub fn create_component_style<E>(
    style: Box<dyn Fn(&Theme<E>) -> Value>,
    component: &'static str,
) -> StyleMap
where
    E: DeserializeOwned + Serialize,
{
    let sheet = create_style_sheet(
        &JsValue::from_serde(&style(&to_theme())).unwrap(),
        component,
        true,
        true,
    );
    to_styles(&sheet)
}

// pub fn create_style_by_struct<T, E>(style: Box<dyn Fn(&Theme<E>) -> T>) -> StyleMap
// where
//     T: DeserializeOwned + Serialize,
//     E: DeserializeOwned + Serialize,
// {
//     let sheet = create_style_sheet(
//         &JsValue::from_serde(&style(&to_theme())).unwrap(),
//         "customer",
//         false,
//         false,
//     );
//     to_styles(&sheet)
// }

// pub fn create_component_style_by_struct<T, E>(
//     style: Box<dyn Fn(&Theme<E>) -> T>,
//     component: &'static str,
// ) -> StyleMap
// where
//     T: DeserializeOwned + Serialize,
//     E: DeserializeOwned + Serialize,
// {
//     let sheet = create_style_sheet(
//         &JsValue::from_serde(&style(&to_theme())).unwrap(),
//         component,
//         false,
//         true,
//     );
//     to_styles(&sheet)
// }
