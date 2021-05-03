use crate::{comp_theme, imports, styles::Extra, to_option};
use web_sys::{Element, Event};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json, remove_element_listener,
    style::{create_component_style, Item},
    theme::{destroy, reload, Theme},
    IframeService,
};
use yewtil::NeqAssign;

#[derive(Deserialize, Serialize)]
pub struct Detail {
    pub _action: String,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| false)]
    pub open: bool,
    #[prop_or_else(|| None)]
    pub heading: Option<String>,
    #[prop_or_else(|| false)]
    pub stacked: bool,
    #[prop_or_else(|| true)]
    pub hide_actions: bool,
    #[prop_or_else(|| true)]
    pub mask_close: bool,
    #[prop_or_default]
    pub closed: Callback<String>,
}

pub type This = Dialog;

comp_theme!(
    Dialog,
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
            let closed = props.closed.to_owned();
            imports("dialog");
            reload::<This>(uuid, link);
            add_element_listener(
                uuid,
                &element,
                "closed",
                Box::new(move |_, e: Event| {
                    let detail = e.into_serde::<Detail>().unwrap();
                    closed.emit(detail._action);
                    IframeService::close();
                }),
            );
        }

        if props.open {
            IframeService::open();
            element.set_attribute("open", "").unwrap();
        } else {
            element.remove_attribute("open").unwrap();
        }
    },
    |uuid: &String, node: &NodeRef| {
        let element = node.cast::<Element>().unwrap();
        remove_element_listener(&uuid, &element, "closed");
        destroy(&uuid);
    },
    |this: &This| {
        let This {
            props, node, uuid, ..
        } = &this;

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "--mdc-dialog-heading-ink-color": theme.dialog.heading_color,
                        "--mdc-dialog-content-ink-color": theme.dialog.content_color,
                        "--mdc-theme-surface": theme.dialog.background,
                    }
                })
            }),
            "Dialog",
        );

        html! {
            <mwc-dialog
                _id=&uuid
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
                heading?=props.heading.as_ref()
                stacked?=to_option(props.stacked)
                hideActions?=to_option(props.hide_actions)
                scrimClickAction?=to_option(!props.mask_close)
            >
                { props.children.to_owned() }
            </mwc-dialog>
        }
    },
    "./doc/dialog.md"
);
