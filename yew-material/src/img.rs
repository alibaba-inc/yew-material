use crate::{comp_theme_data, styles::*};
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::DataMsg,
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_else(|| "0px".into())]
    pub margin: String,
    #[prop_or_else(|| "0px".into())]
    pub padding: String,
    #[prop_or_else(|| "0px".into())]
    pub radius: String,
    #[prop_or_default]
    pub repeat: String,
    #[prop_or_default]
    pub size: String,
    #[prop_or_default]
    pub position: String,
    // #[prop_or_else(|| 1.0)]
    // pub scale: f64,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub loaded: bool,
}

pub type This = Img;

comp_theme_data!(
    Img,
    Data { loaded: false },
    |_, _| {},
    |_, _| {},
    |this: &This| {
        let This {
            props, link, data, ..
        } = &this;
        let pop = props.to_owned();

        let style = create_component_style::<Extra>(
            Box::new(move |_| {
                json!({
                    "jss": {
                        "width": pop.width,
                        "height": pop.height,
                        "margin": pop.margin,
                        "padding": pop.padding,
                        "border-radius": pop.radius,
                        // "transform": match pop.scale as u32 {
                        //     1 => "".into(),
                        //     _ => format!("scale({})", pop.scale),
                        // },
                    },
                    "img_backdrop": {
                        "background": format!("url({})", pop.src),
                        "background-repeat": pop.repeat,
                        "background-size": pop.size,
                        "background-position": pop.position,
                    }
                })
            }),
            "Img",
        );

        let transition_style = match data.loaded {
            false => transition_bg(),
            _ => "".into(),
        };

        let backdrop_style = match data.loaded {
            true => style.item("img_backdrop"),
            _ => transition_bg(),
        };

        let this_link = link.to_owned();
        let onload_callback = Callback::from(move |_| {
            let callback: DataMsg<This> = DataMsg::Callback(Box::new(move |this: &mut This| {
                this.data.loaded = true;
                true
            }));
            this_link.send_message(callback);
        });

        match props.children.len() {
            0 => html! {
                <img
                    id?=props.id.as_ref()
                    class=format!("{} {} {}", props.class, style.theme_item("jss"), transition_style)
                    slot?=props.slot.as_ref()
                    src=props.src
                    onload=&onload_callback
                    onclick=&props.onclick
                />
            },
            _ => html! {
                <div
                    id?=props.id.as_ref()
                    class=format!("{} {} {}", props.class, style.theme_item("jss"), backdrop_style)
                    slot?=props.slot.as_ref()
                    onclick=&props.onclick
                >
                    {
                        match data.loaded {
                            false if props.src != "" => html!{
                                <img
                                    style="width:0px;height:0px"
                                    src=props.src
                                    onload=&onload_callback
                                />
                            },
                            _ => html!{<></>},
                        }
                    }
                    {props.children.to_owned()}
                </div>
            },
        }
    },
    "./doc/img.md"
);
