use crate::{comp_theme, imports, styles::Extra, to_option};
use web_sys::Element;
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{destroy, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_else(|| "".into())]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| None)]
    pub label: Option<String>,
    #[prop_or_else(|| false)]
    pub align_end: bool,
    // #[prop_or_else(|| false)]
    // pub space_between: bool,
    #[prop_or_else(|| false)]
    pub nowrap: bool,
    #[prop_or_else(|| "".into())]
    pub margin: String,
    #[prop_or_else(|| "".into())]
    pub padding: String,
    #[prop_or_else(|| "pointer".into())]
    pub cursor: String,
}

pub type This = FormField;

comp_theme!(
    FormField,
    |this: &This, first: bool| {
        if first {
            imports("formfield");
            let This { uuid, node, .. } = &this;
            let id = uuid.to_string();
            let element = node.cast::<Element>().unwrap();
            WebComponents::try_rendered(
                element,
                Box::new(move |shadow, _| {
                    let style = r#"
                        .mdc-label{
                            cursor: inherit;
                        }"#;
                    WebComponents::set_style(&id, &shadow, style.into());
                }),
            );
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, node, .. } = &this;
        let Props {
            children,
            margin,
            padding,
            cursor,
            ..
        } = props.to_owned();

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "--mdc-theme-text-primary-on-background": theme.color,
                        "margin": margin,
                        "padding": padding,
                        "cursor": cursor,
                    }
                })
            }),
            "FormField",
        );

        html! {
            <mwc-formfield
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", style.theme_item("jss"), props.class)
                slot?=props.slot.as_ref()
                label?=props.label.as_ref()
                alignEnd?=to_option(props.align_end)
                // spaceBetween?=to_option(props.space_between)
                nowrap?=to_option(props.nowrap)
            >
                { children }
            </mwc-formfield>
        }
    },
    "../doc/form/field.md"
);
