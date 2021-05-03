#[cfg(feature = "checkbox")]
use crate::checkbox::style as check_style;

#[cfg(feature = "radio")]
use crate::radio::style as radio_style;

use crate::{comp_theme_data, styles::Extra, to_option};
use web_sys::{Element, ShadowRoot};
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{to_theme, DataMsg, Theme},
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
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub r#type: String,
    #[prop_or_else(|| None)]
    pub value: Option<String>,
    #[prop_or_else(|| None)]
    pub group: Option<String>,
    #[prop_or_else(|| Some(-1))]
    pub tabindex: Option<isize>,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub twoline: bool,
    #[prop_or_else(|| None)]
    pub graphic: Option<String>,
    #[prop_or_else(|| false)]
    pub multiple_graphics: bool,
    #[prop_or_else(|| false)]
    pub has_meta: bool,
    #[prop_or_else(|| false)]
    pub noninteractive: bool,
    #[prop_or_else(|| false)]
    pub selected: bool,
    #[prop_or_else(|| false)]
    pub selector_ignore: bool,
    #[prop_or_else(|| false)]
    pub left: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(|| false)]
    pub activated: bool,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
}

pub type This = ListItem;

comp_theme_data!(
    ListItem,
    Data { shadow: None },
    |this: &This, first: bool| {
        if first {
            let this_link = this.link.to_owned();
            let element = this.node.cast::<Element>().unwrap();
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
    |_, _| {},
    |this: &This| {
        let This {
            props,
            node,
            uuid,
            data,
            ..
        } = &this;
        let (tag, style): (&'static str, String) = match props.r#type.as_str() {
            #[cfg(feature = "radio")]
            "radio" => ("mwc-radio-list-item", radio_style().item("jss")),
            #[cfg(feature = "checkbox")]
            "check" => ("mwc-check-list-item", check_style().item("jss")),
            _ => ("mwc-list-item", "".into()),
        };

        let disabled = props.disabled;
        let self_style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "color": match disabled {
                            true => &theme.disabled,
                            _ => "",
                        },
                        "--mdc-theme-text-primary-on-background":  theme.body.color,
                        "--mdc-theme-text-secondary-on-background": theme.list_item.secondary_color,
                        "--mdc-typography-subtitle1-font-family": theme.font_family,
                        "--mdc-ripple-color": theme.list.ripple,
                        "--mdc-ripple-focus-opacity": 0.12,
                        "&:hover": {
                            "background-color": theme.list.hover,
                        },
                        "&[selectorignore]:focus": {
                            "--mdc-ripple-color": theme.list.hover,
                        },
                        "&[selected]:not([selectorignore]):not([activated])": {
                            "background-color": theme.list.selected,
                        },
                    }
                })
            }),
            "ListItem",
        );

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                        :host(:not([left])) mwc-radio.mdc-deprecated-list-item__meta {{
                            display: inline-table;
                            margin-right: -3.5px;
                        }}
                        :host([activated]) mwc-ripple,
                        :host([activated]) .mdc-deprecated-list-item__text {{
                            color: {};
                            --mdc-ripple-color: {};
                        }}
                    "#,
                    theme.activated, theme.list.ripple
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        html! {
            <@{tag}
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {} {}", props.class, style, self_style.item("jss"))
                slot?=props.slot.as_ref()
                value?=props.value.as_ref()
                group?=props.group.as_ref()
                tabindex?=props.tabindex.as_ref()
                disabled=props.disabled
                twoline?=to_option(props.twoline)
                graphic?=props.graphic.as_ref()
                multipleGraphics?=to_option(props.multiple_graphics)
                hasMeta?=to_option(props.has_meta)
                noninteractive?=to_option(props.noninteractive)
                selected=props.selected
                activated?=to_option(props.activated)
                selectorIgnore?=to_option(props.selector_ignore)
                left?=to_option(props.left)
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </@>

        }
    },
    "../doc/list/item.md"
);
