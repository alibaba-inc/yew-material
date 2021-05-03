use crate::{comp_theme, styles::Extra};
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
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
    pub size: String,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub padding: String,
    #[prop_or_default]
    pub align: String,
    #[prop_or_default]
    pub vertical_align: String,
    #[prop_or_else(|| "1.6".into())]
    pub line_height: String,
    #[prop_or_default]
    pub indent: String,
    #[prop_or_else(|| false)]
    pub auto_wrap: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub type This = Text;

comp_theme!(
    Text,
    |_: &This, _: bool| {},
    |_: &String, _| {},
    |this: &This| {
        let This { props, .. } = &this;
        let pop = props.to_owned();

        let (mut display, mut max_width, mut word_break) = ("inline", "", "normal");

        if props.align != "" {
            display = "block";
        }

        if props.auto_wrap {
            display = "block";
            max_width = "fit-content";
            word_break = "break-all";
        }

        let style = create_component_style::<Extra>(
            Box::new(move |_| {
                json!({
                    "jss": {
                        "display": display,
                        "color": pop.color,
                        "font-size": pop.size,
                        "padding": pop.padding,
                        "max-width": max_width,
                        "text-align": pop.align,
                        "vertical-align": pop.vertical_align,
                        "line-height": pop.line_height,
                        "text-indent": pop.indent,
                        "word-break": word_break,
                    }
                })
            }),
            "Text",
        );

        html! {
            <span
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {}", props.class, style.item("jss"))
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </span>
        }
    },
    "./doc/text.md"
);
