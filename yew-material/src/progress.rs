use crate::{comp_theme, imports, styles::Extra, to_option};
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{destroy, reload, Theme},
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
    #[prop_or_else(|| "circular".into())]
    pub r#type: String,
    #[prop_or_default]
    pub color: String,
    #[prop_or_else(|| "100%".into())]
    pub width: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub size: String,
    #[prop_or_else(|| Some(0))]
    pub density: Option<i32>,
    #[prop_or_else(|| false)]
    pub indeterminate: bool,
    #[prop_or_else(|| Some(0.0))]
    pub progress: Option<f64>,
    #[prop_or_else(|| Some(1.0))]
    pub buffer: Option<f64>,
    #[prop_or_else(|| false)]
    pub reverse: bool,
    #[prop_or_else(|| false)]
    pub closed: bool,
}

pub type This = Progress;

comp_theme!(
    Progress,
    |this: &This, first: bool| {
        if first {
            let This { link, uuid, .. } = &this;
            imports("progress");
            reload::<This>(uuid, link);
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, .. } = &this;
        let pop = props.to_owned();
        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                let color = match pop.color.as_str() {
                    "" => &theme.progress.color,
                    _ => &pop.color,
                };
                json!({
                    "circular": {
                        "display": "inline-flex",
                        "width": pop.size,
                        "height": pop.size,
                        "margin": pop.margin,
                        "align-items": "center",
                        "--mdc-theme-primary": color,
                    },
                    "linear": {
                        "width": pop.width,
                        "margin": pop.margin,
                        "--mdc-linear-progress-buffer-color": theme.progress.linear_buffer,
                        "--mdc-linear-progress-buffering-dots-image": theme.progress.linear_buffer_dots,
                        "--mdc-theme-primary": color,
                    },
                })
            }),
            "Progress",
        );

        let (tag, style) = match props.r#type.as_str() {
            "circular" => ("mwc-circular-progress", style.theme_item("circular")),
            _ => ("mwc-linear-progress", style.theme_item("linear")),
        };

        html! {
            <@{tag}
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, style)
                slot?=props.slot.as_ref()
                indeterminate?=to_option(props.indeterminate)
                progress?=props.progress.as_ref()
                buffer?=props.buffer.as_ref()
                density?=props.density.as_ref()
                reverse?=to_option(props.reverse)
                closed?=to_option(props.closed)
            />
        }
    },
    "./doc/progress.md"
);
