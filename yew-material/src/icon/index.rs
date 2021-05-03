use crate::{comp_theme, imports, styles::Extra};
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{destroy, reload, Theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub icon: String,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub size: String,
    #[prop_or_default]
    pub margin: String,
}

pub type This = Icon;

comp_theme!(
    Icon,
    |this: &This, first: bool| {
        if first {
            imports("icon");
            let This {
                props, link, uuid, ..
            } = &this;
            if props.color == "" {
                reload::<This>(uuid, link);
            }
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, .. } = &this;
        let Props {
            color,
            size,
            margin,
            ..
        } = props.to_owned();

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "--mdc-icon-size": size,
                        "vertical-align": "middle",
                        "margin": margin,
                        "color": if color == "" {
                            &theme.icon.color
                        } else {
                            &color
                        },
                    }
                })
            }),
            "Icon",
        );

        html! {
            <mwc-icon
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
            >
                { &props.icon }
            </mwc-icon>
        }
    },
    "../doc/ico/index.md"
);
