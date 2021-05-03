use crate::{comp_theme_data, imports, styles::Extra, to_option, IconButton};
use web_sys::{Element, ShadowRoot};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, find_element, get_inner_width, json, remove_element_listener,
    set_property,
    style::{create_component_style, Item},
    theme::{destroy, to_theme, DataMsg, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

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
    #[prop_or_else(|| Some(5000))]
    pub timeout: Option<i32>,
    #[prop_or_else(|| None)]
    pub label: Option<String>,
    #[prop_or_default]
    pub align: String,
    #[prop_or_else(|| false)]
    pub stacked: bool,
    #[prop_or_default]
    pub closed: Callback<Element>,
}

fn is_wide() -> bool {
    if get_inner_width() > 600.0 {
        true
    } else {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
}

pub type This = Snackbar;

comp_theme_data!(
    Snackbar,
    Data { shadow: None },
    |this: &This, first: bool| {
        let This {
            props,
            link,
            uuid,
            node,
            ..
        } = &this;
        let element = node.cast::<Element>().unwrap();

        if props.open {
            element.set_attribute("open", "").unwrap();
        } else {
            element.remove_attribute("open").unwrap();
        }

        if first {
            let closed = props.closed.to_owned();
            let align = props.align.to_owned();

            imports("snackbar");

            add_element_listener(
                uuid,
                &element,
                "MDCSnackbar:closed",
                Box::new(move |node: Element, _| {
                    closed.emit(node);
                }),
            );
            add_element_listener(
                uuid,
                &element,
                "MDCSnackbar:opening",
                Box::new(move |ele: Element, _| {
                    let snackbar = find_element(&ele, ".mdc-snackbar");
                    set_property(
                        &snackbar,
                        "bottom",
                        match is_wide() {
                            true => "auto",
                            _ => "0px",
                        },
                    );
                    match align.as_str() {
                        "left" => set_property(&snackbar, "right", "auto"),
                        "right" => set_property(&snackbar, "left", "auto"),
                        _ => {
                            set_property(&snackbar, "right", "0px");
                            set_property(&snackbar, "left", "0px");
                        }
                    };
                    if is_wide() {
                        set_property(&ele, "display", "inherit");
                    }
                }),
            );

            let this_link = link.to_owned();
            WebComponents::try_rendered(
                element,
                Box::new(move |shadow, _| {
                    let mwc_rendered: DataMsg<This> =
                        DataMsg::Callback(Box::new(move |this: &mut This| {
                            this.data.shadow = Some(shadow.to_owned());
                            true
                        }));
                    this_link.send_message(mwc_rendered);
                }),
            );
        }
    },
    |uuid: &String, node: &NodeRef| {
        let element = node.cast::<Element>().unwrap();
        remove_element_listener(&uuid, &element, "closed");
        destroy(&uuid);
    },
    |this: &This| {
        let This {
            props,
            uuid,
            node,
            data,
            ..
        } = &this;
        let self_style = match is_wide() {
            true => create_component_style::<Extra>(
                Box::new(move |_: &Theme<Extra>| {
                    json!({
                        "jss": {
                            "display": "none",
                            "position": "fixed",
                            "top": 0,
                            "left": 0,
                            "right": 0,
                            "z-index": 9,
                        }
                    })
                }),
                "Snackbar",
            )
            .theme_item("jss"),
            _ => "".into(),
        };

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                    .mdc-snackbar .mdc-snackbar__label {{
                        color: {};
                    }}
                    .mdc-snackbar .mdc-snackbar__surface {{
                        background-color: {};
                    }}
                    "#,
                    theme.snackbar.color, theme.snackbar.background,
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        html! {
            <mwc-snackbar
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, self_style)
                stacked?=to_option(props.stacked)
                labelText?=props.label.as_ref()
                timeoutMs?=props.timeout.as_ref()
            >
                { props.children.to_owned() }
                <IconButton slot="dismiss" icon="close"  />
            </mwc-snackbar>
        }
    },
    "./doc/snackbar.md"
);
