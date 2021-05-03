use crate::{comp_theme, imports, styles::Extra, to_option};
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{destroy, reload, Theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| None)]
    pub icon: Option<String>,
    #[prop_or_else(|| false)]
    pub on: bool,
    #[prop_or_else(|| None)]
    pub on_icon: Option<String>,
    #[prop_or_else(|| None)]
    pub off_icon: Option<String>,
    #[prop_or_else(|| None)]
    pub label: Option<String>,
    #[prop_or_default]
    pub size: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub color: String,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub type This = IconButton;

comp_theme!(
    IconButton,
    |this: &This, first: bool| {
        if first {
            imports("icon_button");
            let This {
                props, link, uuid, ..
            } = &this;
            if props.auto_theme {
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
                        "margin": margin,
                        "--mdc-theme-text-disabled-on-light": theme.disabled,
                        "color": if color == "" {
                            &theme.icon.color
                        } else {
                            &color
                        },
                    }
                })
            }),
            "IconButton",
        );

        let tag = match props.on_icon {
            None if props.off_icon == None => "mwc-icon-button",
            _ => "mwc-icon-button-toggle",
        };

        html! {
            <@{tag}
                class=format!("{} {}", props.class, style.theme_item("jss"))
                slot?=props.slot.as_ref()
                icon?=props.icon.as_ref()
                on?=to_option(props.on)
                onIcon?=props.on_icon.as_ref()
                offIcon?=props.off_icon.as_ref()
                label?=props.label.as_ref()
                disabled=props.disabled
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </@>
        }
    },
    "../doc/ico/button.md"
);
