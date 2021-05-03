use crate::{comp_theme, imports, styles::Extra};
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json,
    style::{create_component_style, Item, StyleMap},
    theme::{destroy, reload, Theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_else(|| "".into())]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| None)]
    pub name: Option<String>,
    #[prop_or_else(|| None)]
    pub value: Option<String>,
    #[prop_or_else(|| false)]
    pub checked: bool,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    // #[prop_or_else(|| false)]
    // pub global: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

pub type This = Radio;

pub fn style() -> StyleMap {
    create_component_style::<Extra>(
        Box::new(move |theme: &Theme<Extra>| {
            json!({
                "jss": {
                    "--mdc-theme-secondary": theme.radio.checked,
                    "--mdc-radio-unchecked-color": theme.radio.unchecked,
                    "--mdc-radio-disabled-color": theme.radio.disabled,
                }
            })
        }),
        "Radio",
    )
}

comp_theme!(
    Radio,
    |this: &This, first: bool| {
        let This {
            props,
            link,
            uuid,
            node,
            ..
        } = &this;
        let element = node.cast::<Element>().unwrap();
        if first {
            imports("radio");
            if props.auto_theme {
                reload::<This>(uuid, link);
            }
            let onchange = props.onchange.to_owned();
            let input = node.cast::<HtmlInputElement>().unwrap();
            add_element_listener(
                uuid,
                &element,
                "change",
                Box::new(move |_, _| {
                    onchange.emit(input.checked());
                }),
            );
        }
        if props.checked {
            element.set_attribute("checked", "").unwrap();
        } else {
            element.remove_attribute("checked").unwrap();
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This {
            props, node, uuid, ..
        } = &this;
        html! {
            <mwc-radio
                _id=&uuid
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("__YewMdc_input {} {}", props.class, style().theme_item("jss"))
                slot?=props.slot.as_ref()
                name?=props.name.as_ref()
                value?=props.value.as_ref()
                disabled=props.disabled
                // global?=to_option(props.global)
                onclick=&props.onclick
            />
        }
    },
    "./doc/radio.md"
);
