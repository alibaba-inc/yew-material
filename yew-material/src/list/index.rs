use crate::{comp_theme, imports, styles::Extra, to_option};
use web_sys::Element;
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json, off, on, remove_children_attr,
    style::{create_component_style, Item},
    theme::{destroy, reload, Theme},
};
use yewtil::NeqAssign;

pub type SelectedRes = (Vec<i32>, Vec<i32>, Vec<i32>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Detail<T> {
    pub _detail: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleSelectedRes {
    pub index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultiSelectedRes {
    pub diff: Diff,
    pub index: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Diff {
    pub added: Vec<i32>,
    pub removed: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| false)]
    pub root_tabbable: bool,
    #[prop_or_else(|| false)]
    pub multi: bool,
    #[prop_or_else(|| false)]
    pub noninteractive: bool,
    #[prop_or_else(|| false)]
    pub selected_style: bool,
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub item_height: String,
    #[prop_or_default]
    pub item_graphic_margin: String,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onselected: Callback<SelectedRes>,
    #[prop_or_else(|| false)]
    pub activatable: bool,
    // #[prop_or_else(|| false)]
    // pub wrap_focus: bool,
    // #[prop_or_else(|| None)]
    // pub item_roles: Option<String>,
    // #[prop_or_else(|| None)]
    // pub inner_role: Option<String>,
    // #[prop_or_else(|| vec![0])]
    // pub items: Vec<i32>,
    // #[prop_or_else(|| vec![0])]
    // pub selected: Vec<i32>,
    // #[prop_or_else(|| Some(-1))]
    // pub index: Option<isize>,
}

pub type This = List;

comp_theme!(
    List,
    |this: &This, first: bool| {
        if first {
            imports("list");
            let This {
                props,
                link,
                uuid,
                node,
                ..
            } = &this;
            if props.auto_theme {
                reload::<This>(uuid, link);
            }

            let element = node.cast::<Element>().unwrap();
            if props.selected_style {
                let ele = element.to_owned();
                on(
                    uuid,
                    &element,
                    "mwc-list-item",
                    "click",
                    Box::new(move |item: Element| {
                        remove_children_attr(&ele, "mwc-list-item", "selected");
                        item.set_attribute("selected", "").unwrap();
                    }),
                );
            }

            let multi = props.multi;
            let onselected = props.onselected.to_owned();
            add_element_listener(
                uuid,
                &element,
                "selected",
                Box::new(move |_, e: Event| {
                    match multi {
                        true => {
                            let res = e.into_serde::<Detail<MultiSelectedRes>>().unwrap();
                            onselected.emit((
                                res._detail.index,
                                res._detail.diff.added,
                                res._detail.diff.removed,
                            ));
                        }
                        _ => {
                            let res = e.into_serde::<Detail<SingleSelectedRes>>().unwrap();
                            onselected.emit((vec![res._detail.index], vec![-1], vec![-1]));
                        }
                    };
                }),
            );
        }
    },
    |uuid: &String, node: &NodeRef| {
        let element = node.cast::<Element>().unwrap();
        off(&uuid, &element, "click");
        destroy(&uuid);
    },
    |this: &This| {
        let This { props, node, .. } = &this;
        let Props {
            children,
            width,
            item_height,
            item_graphic_margin,
            ..
        } = props.to_owned();
        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "width": width,
                        "background": theme.list.background,
                        "--mdc-list-item-graphic-margin": item_graphic_margin,
                        "& [mwc-list-item]": {
                            "height": item_height,
                        },
                        "& a": {
                            "text-decoration": "none"
                        }
                    }
                })
            }),
            "List",
        );
        html! {
            <mwc-list
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
                slot?=props.slot.as_ref()
                rootTabbable?=to_option(props.root_tabbable)
                multi?=to_option(props.multi)
                noninteractive?=to_option(props.noninteractive)
                activatable?=to_option(props.activatable)
                // index?=props.index.as_ref()
                // wrapFocus?=to_option(props.wrap_focus)
                // itemRoles?=props.item_roles.as_ref()
                // innerRole?=props.inner_role.as_ref()
            >
                { children }
            </mwc-list>

        }
    },
    "../doc/list/index.md"
);
