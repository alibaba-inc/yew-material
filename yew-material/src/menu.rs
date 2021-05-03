use crate::{
    comp_theme_data, imports,
    list::index::{Detail, SingleSelectedRes},
    styles::Extra,
    to_option,
};
use web_sys::{Element, ShadowRoot};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, anchor, json,
    style::{create_component_style, Item},
    theme::{destroy, to_theme, DataMsg, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub anchor: Html,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| false)]
    pub full_width: bool,
    #[prop_or_else(|| false)]
    pub fixed: bool,
    // #[prop_or_else(|| false)]
    // pub multi: bool,
    #[prop_or_else(|| Some("BOTTOM_LEFT".into()))]
    pub corner: Option<String>,
    #[prop_or_default]
    pub onselected: Callback<u32>,
    // pub onselected: Callback<SelectedRes>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
    pub open: bool,
}

pub type This = Menu;

comp_theme_data!(
    Menu,
    Data {
        shadow: None,
        open: false
    },
    |this: &This, first: bool| {
        if first {
            imports("menu");
            let This {
                props,
                link,
                uuid,
                node,
                ..
            } = &this;
            let element = node.cast::<Element>().unwrap();

            anchor(&element, &format!("{}-for-anchor", uuid));

            // let multi = props.multi;
            let onselected = props.onselected.to_owned();
            add_element_listener(
                uuid,
                &element,
                "selected",
                Box::new(move |_, e: Event| {
                    let res = e.into_serde::<Detail<SingleSelectedRes>>().unwrap();
                    onselected.emit(res._detail.index as u32);
                    // match multi {
                    //     true => {
                    //         let res = e.into_serde::<Detail<MultiSelectedRes>>().unwrap();
                    //         onselected.emit((
                    //             res._detail.index,
                    //             res._detail.diff.added,
                    //             res._detail.diff.removed,
                    //         ));
                    //     }
                    //     _ => {
                    //         let res = e.into_serde::<Detail<SingleSelectedRes>>().unwrap();
                    //         onselected.emit((vec![res._detail.index], vec![-1], vec![-1]));
                    //     }
                    // };
                }),
            );

            let this_link = link.to_owned();
            add_element_listener(
                &uuid,
                &element,
                "closed",
                Box::new(move |_, _| {
                    let set_status: DataMsg<This> =
                        DataMsg::Callback(Box::new(move |this: &mut This| {
                            this.data.open = false;
                            false
                        }));
                    this_link.send_message(set_status);
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
    |uuid: &String, _| { destroy(&uuid) },
    |this: &This| {
        let This {
            props,
            uuid,
            data,
            node,
            link,
            ..
        } = &this;

        let style = create_component_style::<Extra>(
            Box::new(move |_: &Theme<Extra>| {
                json!({
                    "jss": {
                        "position": "relative",
                        "display": "inline-flex",
                    },
                })
            }),
            "Menu",
        );

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                    mwc-menu-surface {{
                        --mdc-theme-surface: {};
                    }}
                    "#,
                    theme.list.background,
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        let this_link = link.to_owned();
        let this_node = node.to_owned();
        html! {
            <div
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
            >
                <div
                    id=format!("{}-for-anchor", uuid)
                    onclick=Callback::from(move |_| {
                        let element = this_node.cast::<Element>().unwrap();
                        let set_status: DataMsg<This> = DataMsg::Callback(Box::new(move |this: &mut This| {
                            this.data.open = !this.data.open;
                            if this.data.open {
                                element.set_attribute("open", "").unwrap();
                            } else {
                                element.remove_attribute("open").unwrap();
                            }
                            false
                        }));
                        this_link.send_message(set_status);
                    })
                >
                    {props.anchor.to_owned()}
                </div>
                <mwc-menu
                    ref=node.to_owned()
                    fullwidth?=to_option(props.full_width)
                    fixed?=to_option(props.fixed)
                    corner?=props.corner.as_ref()
                    activatable=""
                >
                    {props.children.to_owned()}
                </mwc-menu>
            </div>
        }
    },
    "./doc/menu.md"
);
