use crate::{comp_theme, styles::Extra, to_option};
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::Theme,
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
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| None)]
    pub label: Option<String>,
    #[prop_or_else(|| None)]
    pub icon: Option<String>,
    #[prop_or_else(|| false)]
    pub has_image_icon: bool,
    #[prop_or_else(|| None)]
    pub indicator_icon: Option<String>,
    #[prop_or_else(|| false)]
    pub is_fading_indicator: bool,
    #[prop_or_else(|| false)]
    pub min_width: bool,
    #[prop_or_else(|| false)]
    pub is_min_width_indicator: bool,
    #[prop_or_else(|| false)]
    pub stacked: bool,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub type This = TabItem;

comp_theme!(
    TabItem,
    |_, _| {},
    |_, _| {},
    |this: &This| {
        let This { props, node, .. } = &this;

        let disabled = props.disabled;
        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "pointer-events":  match disabled {
                            true => "none",
                            _ => "",
                        },
                        "--mdc-theme-primary": theme.tab_item.activated,
                        "--mdc-tab-text-label-color-default": match disabled {
                            true => &theme.disabled,
                            _ => &theme.tab_item.color,
                        },
                        "--mdc-tab-color-default": match disabled {
                            true => &theme.disabled,
                            _ => &theme.tab_item.icon_color,
                        },
                        "--mdc-ripple-color": theme.tab_item.ripple,
                    }
                })
            }),
            "TabItem",
        );

        html! {
            <mwc-tab
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, style.item("jss"))
                slot?=props.slot.as_ref()
                label?=props.label.as_ref()
                icon?=props.icon.as_ref()
                hasImageIcon?=to_option(props.has_image_icon)
                indicatorIcon?=props.indicator_icon.as_ref()
                isFadingIndicator?=to_option(props.is_fading_indicator)
                minWidth?=to_option(props.min_width)
                isMinWidthIndicator?=to_option(props.is_min_width_indicator)
                stacked?=to_option(props.stacked)
                disabled=props.disabled
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </mwc-tab>

        }
    },
    "../doc/tab/item.md"
);
