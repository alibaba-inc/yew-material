use crate::{comp_theme, filter_zero, styles::*, to_pixel};
use yew::prelude::*;
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
    #[prop_or_else(|| "".into())]
    pub r#type: String,
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub padding: String,
    #[prop_or_default]
    pub position: String,
    #[prop_or_default]
    pub top: String,
    #[prop_or_default]
    pub left: String,
    #[prop_or_default]
    pub bottom: String,
    #[prop_or_default]
    pub right: String,
    #[prop_or_default]
    pub opacity: String,
    #[prop_or_else(|| 0)]
    pub deg: i32,
    #[prop_or_else(|| false)]
    pub scrollbar_x: bool,
    #[prop_or_else(|| false)]
    pub scrollbar_y: bool,
    #[prop_or_else(|| 0.3)]
    pub time: f32,
    #[prop_or_else(|| false)]
    pub every_time: bool,
    #[prop_or_else(|| 0)]
    pub index: u32,
    #[prop_or_default]
    pub identity: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub type This = Animate;

comp_theme!(
    Animate,
    |_, _| {},
    |_, _| {},
    |this: &This| {
        let This { props, .. } = &this;
        let pop = props.to_owned();
        let mut will = props.r#type.to_owned();
        let mut style_opacity = "".into();
        let mut indent = if props.every_time { "" } else { "once" };
        let mut overflow_x = if props.scrollbar_x { "auto" } else { "hidden" };
        let mut overflow_y = if props.scrollbar_y { "auto" } else { "hidden" };

        if !props.scrollbar_x && !props.scrollbar_y {
            overflow_x = "";
            overflow_y = "";
        }

        if props.identity != "" {
            indent = &props.identity;
        }

        if props.r#type == "opacity" && props.opacity == "" {
            style_opacity = animation(indent.into(), "opacity", "0", "1", props.time);
            will = "animation".to_string();
        }

        if props.r#type == "rotate" {
            will = "transform".to_string();
        }

        let style_scrollbar = if props.scrollbar_x || props.scrollbar_y {
            style("scrollbar")
        } else {
            "".into()
        };

        let transition = match props.r#type.as_str() {
            "" => "".into(),
            _ => format!("{} {}s", will, props.time),
        };

        let transform = match props.deg {
            0 => "".into(),
            _ => format!("rotate({}deg)", props.deg),
        };

        let self_style = create_component_style::<Extra>(
            Box::new(move |_| {
                json!({
                    "jss": {
                        "width": to_pixel(&pop.width),
                        "height": to_pixel(&pop.height),
                        "margin": pop.margin,
                        "padding": pop.padding,
                        "position": pop.position,
                        "top": pop.top,
                        "left": pop.left,
                        "bottom": pop.bottom,
                        "right": pop.right,
                        "opacity": pop.opacity,
                        "will-change": will,
                        "overflow-x": overflow_x,
                        "overflow-y": overflow_y,
                        "transform": transform,
                        "transition": transition,
                        "z-index": filter_zero(pop.index),
                    }
                })
            }),
            "Animate",
        );

        html! {
            <div
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {} {} {}", props.class, self_style.item("jss"), style_opacity, style_scrollbar)
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </div>
        }
    },
    "./doc/animate.md"
);
