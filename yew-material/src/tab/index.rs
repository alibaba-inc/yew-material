use crate::{
    comp_theme, imports,
    list::index::{Detail, SingleSelectedRes},
    styles::Extra,
};
use web_sys::Element;
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json,
    style::{create_component_style, Item},
    theme::{destroy, Theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub width: String,
    #[prop_or_else(|| Some(0))]
    pub active_index: Option<u32>,
    #[prop_or_default]
    pub onactivated: Callback<u32>,
}

pub type This = Tab;

comp_theme!(
    Tab,
    |this: &This, first: bool| {
        if first {
            imports("tab");
            let This {
                props, uuid, node, ..
            } = &this;
            let element = node.cast::<Element>().unwrap();
            let onactivated = props.onactivated.to_owned();
            add_element_listener(
                uuid,
                &element,
                "MDCTabBar:activated",
                Box::new(move |_, e: Event| {
                    let res = e.into_serde::<Detail<SingleSelectedRes>>().unwrap();
                    onactivated.emit(res._detail.index as u32);
                }),
            );
        }
    },
    |uuid: &String, _| { destroy(&uuid) },
    |this: &This| {
        let This { props, node, .. } = &this;

        let width = props.width.to_string();
        let style = create_component_style::<Extra>(
            Box::new(move |_: &Theme<Extra>| {
                json!({
                    "jss": {
                        "width": width,
                    },
                })
            }),
            "Tab",
        );

        html! {
            <mwc-tab-bar
                ref=node.to_owned()
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
                activeIndex?=props.active_index.as_ref()
            >
                {props.children.to_owned()}
            </mwc-tab-bar>
        }
    },
    "../doc/tab/index.md"
);
