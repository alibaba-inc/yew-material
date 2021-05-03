use crate::{comp_theme, imports, styles::Extra};
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json,
    style::{create_component_style, Item},
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
    #[prop_or_else(|| Some("on".into()))]
    pub on_value: Option<String>,
    #[prop_or_else(|| Some("off".into()))]
    pub off_value: Option<String>,
    #[prop_or_else(|| false)]
    pub checked: bool,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onchange: Callback<(bool, Event)>,
}

pub type This = Switch;

comp_theme!(
    Switch,
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
            imports("switch");
            if props.auto_theme {
                reload::<This>(uuid, link);
            }

            let onchange = props.onchange.to_owned();
            let input = node.cast::<HtmlInputElement>().unwrap();
            add_element_listener(
                uuid,
                &element,
                "change",
                Box::new(move |_, e: Event| {
                    onchange.emit((input.checked(), e));
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
        let This { props, node, .. } = &this;

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "--mdc-theme-secondary": theme.switch.checked,
                        "--mdc-theme-surface": theme.switch.unchecked,
                        "--mdc-theme-on-surface	": theme.switch.disabled,
                    }
                })
            }),
            "Switch",
        );

        html! {
            <mwc-switch
                _type="switch"
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("__YewMdc_input {} {}", props.class, style.theme_item("jss"))
                slot?=props.slot.as_ref()
                name?=props.name.as_ref()
                value?=props.on_value.as_ref()
                off_value?=props.off_value.as_ref()
                disabled=props.disabled
            />
        }
    },
    "./doc/switch.md"
);
