use crate::{comp_theme_data, imports, styles::Extra, to_option};
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json,
    style::{create_component_style, Item, StyleMap},
    theme::{destroy, reload_with_data, DataMsg, Theme},
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
    #[prop_or_else(|| None)]
    pub checked: Option<bool>,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub indeterminate: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub checked: bool,
}

pub type This = Checkbox;

pub fn style() -> StyleMap {
    create_component_style::<Extra>(
        Box::new(move |theme: &Theme<Extra>| {
            json!({
                "jss": {
                    "--mdc-theme-secondary": theme.checkbox.checked,
                    "--mdc-checkbox-unchecked-color": theme.checkbox.unchecked,
                    "--mdc-checkbox-disabled-color": theme.checkbox.disabled,
                }
            })
        }),
        "Checkbox",
    )
}

comp_theme_data!(
    Checkbox,
    Data { checked: false },
    |this: &mut This, first: bool| {
        let This {
            props,
            link,
            uuid,
            node,
            ..
        } = &this;
        let element = node.cast::<Element>().unwrap();

        if first {
            imports("checkbox");

            if props.auto_theme {
                reload_with_data::<This>(uuid, link);
            }

            let this_link = link.to_owned();
            let onchange = props.onchange.to_owned();
            let input = node.cast::<HtmlInputElement>().unwrap();
            add_element_listener(
                uuid,
                &element,
                "change",
                Box::new(move |_, _| {
                    // let element = find_element(&ele, ".mdc-checkbox__native-control");
                    // let input = element.dyn_into::<HtmlInputElement>().unwrap();
                    let checked = input.checked();
                    let mwc_rendered: DataMsg<This> =
                        DataMsg::Callback(Box::new(move |this: &mut This| {
                            this.data.checked = checked;
                            false
                        }));
                    this_link.send_message(mwc_rendered);
                    onchange.emit(checked);
                }),
            );
        }
        match props.checked {
            Some(checked) => {
                if checked {
                    element.set_attribute("checked", "").unwrap();
                } else {
                    element.remove_attribute("checked").unwrap();
                }
            }
            _ => {
                if this.data.checked {
                    element.set_attribute("checked", "").unwrap();
                } else {
                    element.remove_attribute("checked").unwrap();
                }
            }
        };
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, node, .. } = &this;
        html! {
            <mwc-checkbox
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("__YewMdc_input {} {}", props.class, style().theme_item("jss"))
                slot?=props.slot.as_ref()
                name?=props.name.as_ref()
                value?=props.value.as_ref()
                disabled=props.disabled
                indeterminate?=to_option(props.indeterminate)
            />
        }
    },
    "./doc/checkbox.md"
);
