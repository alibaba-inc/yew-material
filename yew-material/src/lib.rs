#![feature(external_doc)]

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

pub mod styles;
// pub use self::styles::*;

#[cfg(feature = "animate")]
pub mod animate;
#[cfg(feature = "animate")]
pub use self::animate::*;

#[cfg(feature = "appbar")]
pub mod appbar;
#[cfg(feature = "appbar")]
pub use self::appbar::*;

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "button")]
pub use self::button::*;

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dialog")]
pub use self::dialog::*;

#[cfg(feature = "divider")]
pub mod divider;
#[cfg(feature = "divider")]
pub use self::divider::*;

#[cfg(feature = "flex")]
pub mod flex;
#[cfg(feature = "flex")]
pub use self::flex::*;

#[cfg(any(feature = "form", feature = "formfield"))]
pub mod form;
#[cfg(feature = "formfield")]
pub use self::form::field::*;
#[cfg(feature = "form")]
pub use self::form::index::*;

#[cfg(any(feature = "icon", feature = "icon_button", feature = "button"))]
pub mod icon;
#[cfg(feature = "icon_button")]
pub use self::icon::button::*;
#[cfg(any(feature = "icon", feature = "button"))]
pub use self::icon::index::*;

#[cfg(feature = "img")]
pub mod img;
#[cfg(feature = "img")]
pub use self::img::*;

#[cfg(feature = "innerhtml")]
pub mod innerhtml;
#[cfg(feature = "innerhtml")]
pub use self::innerhtml::*;

#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "list")]
pub use self::list::*;

#[cfg(any(feature = "progress", feature = "button"))]
pub mod progress;
#[cfg(any(feature = "progress", feature = "button"))]
pub use self::progress::*;

#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "radio")]
pub use self::radio::*;

#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "checkbox")]
pub use self::checkbox::*;

#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
pub use self::switch::*;

#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "slider")]
pub use self::slider::*;

#[cfg(feature = "skeleton")]
pub mod skeleton;
#[cfg(feature = "skeleton")]
pub use self::skeleton::*;

#[cfg(feature = "snackbar")]
pub mod snackbar;
#[cfg(feature = "snackbar")]
pub use self::snackbar::*;

#[cfg(feature = "text")]
pub mod text;
#[cfg(feature = "text")]
pub use self::text::*;

#[cfg(feature = "textfield")]
pub mod textfield;
#[cfg(feature = "textfield")]
pub use self::textfield::*;

#[cfg(feature = "textarea")]
pub mod textarea;
#[cfg(feature = "textarea")]
pub use self::textarea::*;

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "select")]
pub use self::select::*;

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menu")]
pub use self::menu::*;

#[cfg(feature = "tab")]
pub mod tab;
#[cfg(feature = "tab")]
pub use self::tab::*;

#[cfg(feature = "iframe")]
pub mod iframe;
#[cfg(feature = "iframe")]
pub use self::iframe::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = "Imports")]
    pub fn imports(name: &str);
}

fn to_option(value: bool) -> Option<&'static str> {
    match value {
        true => Some(""),
        false => None,
    }
}

fn filter_zero(value: u32) -> String {
    match value {
        0 => "".into(),
        _ => value.to_string(),
    }
}

fn to_pixel(pixel: &String) -> String {
    if *pixel == "" {
        "auto".into()
    } else {
        pixel.into()
    }
}

#[macro_export]
macro_rules! comp_theme {
    ($this: ident, $rendered: expr, $destroy: expr, $html: expr, $doc: expr) => {
        #[allow(dead_code)]
        #[doc(include = $doc)]
        pub struct $this {
            uuid: String,
            props: Props,
            link: ComponentLink<Self>,
            node: yew::NodeRef,
        }

        impl yew::Component for $this {
            type Message = yew_material_utils::theme::Msg;
            type Properties = Props;

            fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
                Self {
                    uuid: uuid::Uuid::new_v4().to_string(),
                    props,
                    link,
                    node: yew::NodeRef::default(),
                }
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Self::Message::Reload => true,
                }
            }

            fn change(&mut self, props: Self::Properties) -> bool {
                self.props.neq_assign(props)
            }

            fn rendered(&mut self, first: bool) {
                $rendered(&self, first);
            }

            fn destroy(&mut self) {
                $destroy(&self.uuid, &self.node);
            }

            fn view(&self) -> Html {
                $html(&self)
            }
        }
    };
}

#[macro_export]
macro_rules! comp_theme_data {
    ($this: ident, $data: expr, $rendered: expr, $destroy: expr, $html: expr, $doc: expr) => {
        #[doc(include = $doc)]
        #[derive(Clone)]
        pub struct $this {
            uuid: String,
            props: Props,
            link: ComponentLink<Self>,
            node: yew::NodeRef,
            data: Data,
        }

        impl yew::Component for $this {
            type Message = yew_material_utils::theme::DataMsg<Self>;
            type Properties = Props;

            fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
                Self {
                    uuid: uuid::Uuid::new_v4().to_string(),
                    props,
                    link,
                    node: yew::NodeRef::default(),
                    data: $data,
                }
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Self::Message::Callback(callback) => callback(self),
                    Self::Message::Reload => true,
                }
            }

            fn change(&mut self, props: Self::Properties) -> bool {
                self.props.neq_assign(props)
            }

            fn rendered(&mut self, first: bool) {
                $rendered(self, first);
            }

            fn destroy(&mut self) {
                $destroy(&self.uuid, &self.node);
            }

            fn view(&self) -> Html {
                $html(&self)
            }
        }
    };
}

pub mod prelude {

    #[cfg(feature = "animate")]
    pub use crate::animate::*;

    #[cfg(feature = "appbar")]
    pub use crate::appbar::*;

    #[cfg(feature = "button")]
    pub use crate::button::*;

    #[cfg(feature = "dialog")]
    pub use crate::dialog::*;

    #[cfg(feature = "divider")]
    pub use crate::divider::*;

    #[cfg(feature = "flex")]
    pub use crate::flex::*;

    #[cfg(feature = "form")]
    pub use crate::form::index::*;

    #[cfg(feature = "formfield")]
    pub use crate::form::field::*;

    #[cfg(any(feature = "icon", feature = "button"))]
    pub use crate::icon::index::*;

    #[cfg(feature = "icon_button")]
    pub use crate::icon::button::*;

    #[cfg(feature = "img")]
    pub use crate::img::*;

    #[cfg(feature = "innerhtml")]
    pub use crate::innerhtml::*;

    #[cfg(feature = "list")]
    pub use crate::list::*;

    #[cfg(any(feature = "progress", feature = "button"))]
    pub use crate::progress::*;

    #[cfg(feature = "radio")]
    pub use crate::radio::*;

    #[cfg(feature = "checkbox")]
    pub use crate::checkbox::*;

    #[cfg(feature = "switch")]
    pub use crate::switch::*;

    #[cfg(feature = "slider")]
    pub use crate::slider::*;

    #[cfg(feature = "skeleton")]
    pub use crate::skeleton::*;

    #[cfg(feature = "snackbar")]
    pub use crate::snackbar::*;

    #[cfg(feature = "text")]
    pub use crate::text::*;

    #[cfg(feature = "textfield")]
    pub use crate::textfield::*;

    #[cfg(feature = "textarea")]
    pub use crate::textarea::*;

    #[cfg(feature = "select")]
    pub use crate::select::*;

    #[cfg(feature = "menu")]
    pub use crate::menu::*;

    #[cfg(feature = "tab")]
    pub use crate::tab::*;

    #[cfg(feature = "iframe")]
    pub use crate::iframe::*;

    pub use yew_material_macro::FormRes;
}

pub use self::prelude::*;
