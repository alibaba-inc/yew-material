use crate::{comp_theme, styles::*, to_pixel};
use yew::prelude::*;
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
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub max_width: String,
    #[prop_or_default]
    pub max_height: String,
    #[prop_or_default]
    pub min_width: String,
    #[prop_or_default]
    pub min_height: String,
    #[prop_or_else(|| true)]
    pub toggle: bool,
    #[prop_or_else(|| false)]
    pub animate: bool,
    #[prop_or_else(|| "vertical".into())]
    pub animate_type: String,
    #[prop_or_else(|| false)]
    pub hide: bool,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub padding: String,
    #[prop_or_else(|| 0)]
    pub grow: u32,
    #[prop_or_else(|| "row".into())]
    pub direction: String,
    #[prop_or_else(|| "flex-start".into())]
    pub justify: String,
    #[prop_or_else(|| "stretch".into())]
    pub align: String,
    #[prop_or_else(|| false)]
    pub border: bool,
    #[prop_or_else(|| false)]
    pub border_top: bool,
    #[prop_or_else(|| false)]
    pub border_bottom: bool,
    #[prop_or_else(|| false)]
    pub border_left: bool,
    #[prop_or_else(|| false)]
    pub border_right: bool,
    #[prop_or_else(|| false)]
    pub scrollbar_x: bool,
    #[prop_or_else(|| false)]
    pub scrollbar_y: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

fn to_toggle(w_h_pixel: String, w_h_name: &'static str, props: &Props) -> (String, &'static str) {
    (
        //匹配切换状态，如果是展开，返回原宽高，如果是关闭，强制返回0像素
        match props.toggle {
            true => w_h_pixel,
            _ => "0px!important".into(),
        },
        //匹配动画状态，如果开启，返回宽高名，如果关闭，返回none，让动画失效
        match props.animate {
            true => w_h_name,
            _ => "none",
        },
    )
}

fn to_overflow(scrollbar: bool, w_h_pixel: &String, props: &Props) -> &'static str {
    match scrollbar {
        //宽或高强制为0像素，则隐藏
        _ if w_h_pixel == "0px!important" => "hidden",
        //修正火狐下空节点hidden属性宽高计算问题
        _ if props.animate && props.children.len() != 0 => "hidden",
        //没有开启滚动条则不设置overflow属性
        _ if !props.scrollbar_x && !props.scrollbar_y => "",
        //开启了滚动条则设置为自适应
        true => "auto",
        //默认为隐藏
        _ => "hidden",
    }
}

pub type This = Flex;

comp_theme!(
    Flex,
    |this: &This, first: bool| {
        if first {
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
        let pop = props.to_owned();

        let (mut h, mut w) = (to_pixel(&props.height), to_pixel(&props.width));

        let shrink = match props.grow {
            0 if props.width != "" || props.height != "" => 0,
            _ => 1,
        };

        let will = match props.animate_type.as_str() {
            "horizontal" => {
                let (width, status) = to_toggle(w, "width", props);
                w = width;
                status
            }
            _ => {
                let (height, status) = to_toggle(h, "height", props);
                h = height;
                status
            }
        };

        let overflow_x = to_overflow(props.scrollbar_x, &w, props);
        let overflow_y = to_overflow(props.scrollbar_y, &h, props);

        let display = match props.hide {
            true => "none",
            _ => "flex",
        };

        let solid = |status: bool, color: &str| -> String {
            match status {
                true => format!("1px solid {}", color),
                _ => "".into(),
            }
        };

        let style_scrollbar = if props.scrollbar_x || props.scrollbar_y {
            style("scrollbar")
        } else {
            "".into()
        };

        let self_style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                let color = &theme.divider.color;
                json!({
                    "jss": {
                        "width": w,
                        "height": h,
                        "max-width": pop.max_width,
                        "max-height": pop.max_height,
                        "min-width": pop.min_width,
                        "min-height": pop.min_height,
                        "margin": pop.margin,
                        "padding": pop.padding,
                        "will-change": will,
                        "display": display,
                        "flex-grow": pop.grow,
                        "transition": format!("{} 0.3s", will),
                        "flex-direction": pop.direction,
                        "justify-content": pop.justify,
                        "align-items": pop.align,
                        "flex-shrink": shrink,
                        "border": solid(pop.border, &color),
                        "border-top": solid(pop.border_top, &color),
                        "border-bottom": solid(pop.border_bottom, &color),
                        "border-left": solid(pop.border_left, &color),
                        "border-right": solid(pop.border_right, &color),
                        "overflow-x": overflow_x,
                        "overflow-y": overflow_y,
                    }
                })
            }),
            "Flex",
        );

        html! {
            <div
                id?=props.id.as_ref()
                slot?=props.slot.as_ref()
                class=format!("{} {} {}", props.class, self_style.item("jss"), style_scrollbar)
                onclick=&props.onclick
            >
                { props.children.to_owned() }
            </div>
        }
    },
    "./doc/flex.md"
);
