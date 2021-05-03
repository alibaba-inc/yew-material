use crate::{loader::script, PUBLIC_URL};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub type Editor = JsValue;

#[macro_export]
macro_rules! ck_editor_config_init {
    () => {
        #[wasm_bindgen(module = "/src/plugins_config/ck_editor.js")]
        extern "C" {
            #[wasm_bindgen(js_name = "config")]
            pub fn get_ck_editor_config(param: String) -> JsValue;
        }
    };
}

#[wasm_bindgen(module = "/src/plugins/ck_editor.js")]
extern "C" {
    fn init_editor(selector: &str, config: JsValue, ready: &Closure<dyn Fn(Editor)>);
    pub fn set_data(editor: &Editor, text: &str);
    pub fn get_data(editor: &Editor) -> String;
}

pub fn default() -> Editor {
    "".into()
}

pub async fn init(id: &str, config: JsValue, callback: Callback<Editor>) {
    script(
        &(PUBLIC_URL.to_string() + "/plugins/ck_editor/ckeditor.js"),
        false,
    )
    .await;

    let ready = Closure::wrap(Box::new(move |e: Editor| {
        callback.emit(e);
    }) as Box<dyn Fn(Editor)>);

    init_editor(id, config, &ready);

    ready.forget();
}
