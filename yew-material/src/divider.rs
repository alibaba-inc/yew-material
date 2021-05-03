use crate::{comp_theme, styles::Extra, to_option};
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{destroy, reload, to_theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_else(|| false)]
    pub vertical: bool,
    #[prop_or_else(|| false)]
    pub padded: bool,
    #[prop_or_else(|| false)]
    pub inset: bool,
    #[prop_or_default]
    pub role: String,
}

pub type This = Divider;

comp_theme!(
    Divider,
    |this: &This, first: bool| {
        if first {
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

        let theme = to_theme::<Extra>();
        let color = match props.color.as_str() {
            "" => &theme.divider.color,
            _ => &props.color,
        };
        let margin = match props.margin.as_str() {
            "" if props.vertical => "0 10px",
            "" if !props.vertical => "15px 0",
            _ => &props.margin,
        };

        let jss_json = match props.vertical {
            _ if props.role == "list_item" => json!({
                "jss": {
                    "margin-top": "-1px",
                    "border-color": color,
                }
            }),
            true => json!({
                "jss": {
                    "position": "relative",
                    "display": "inline-block",
                    "width": "0px",
                    "min-height": ".9em",
                    "top": "-.06em",
                    "vertical-align": "middle",
                    "margin": margin,
                    "border-left": "1px solid",
                    "border-color": color,
                }
            }),
            _ => json!({
                "jss": {
                    "display": "flex",
                    "clear": "both",
                    "min-width": "100%",
                    "margin": margin,
                    "border-top": "1px solid",
                    "border-color": color,
                }
            }),
        };

        let style =
            create_component_style::<Extra>(Box::new(move |_| jss_json.to_owned()), "Divider");

        let role = match props.role.as_str() {
            "list_item" => Some("separator"),
            _ => None,
        };

        html! {
            <div
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
                divider=true
                padded?=to_option(props.padded)
                inset?=to_option(props.inset)
                role?=role.as_ref()
            />
        }
    },
    "./doc/divider.md"
);
