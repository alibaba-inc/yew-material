use crate::{comp_theme_data, imports, list::Detail, styles::Extra, to_option};
use web_sys::{Element, ShadowRoot};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, json,
    style::{create_component_style, Item},
    theme::{destroy, to_theme, DataMsg, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

#[derive(Debug, Deserialize, Serialize)]
pub struct Value {
    pub _value: i32,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_else(|| "".into())]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| Some(0))]
    pub value: Option<i32>,
    #[prop_or_default]
    pub width: String,
    #[prop_or_else(|| Some(0))]
    pub min: Option<i32>,
    #[prop_or_else(|| Some(100))]
    pub max: Option<i32>,
    #[prop_or_else(|| Some(1))]
    pub step: Option<i32>,
    #[prop_or_else(|| false)]
    pub pin: bool,
    #[prop_or_else(|| false)]
    pub markers: bool,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    // #[prop_or_else(|| false)]
    // pub auto_theme: bool,
    #[prop_or_default]
    pub onchange: Callback<i32>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
}

pub type This = Slider;

comp_theme_data!(
    Slider,
    Data { shadow: None },
    |this: &This, first: bool| {
        let This {
            props,
            link,
            uuid,
            node,
            ..
        } = &this;
        let element = node.cast::<Element>().unwrap();
        if first {
            imports("slider");
            // if props.markers {
            //reload_with_data::<This>(&uuid, link);
            // }

            let onchange = props.onchange.to_owned();
            add_element_listener(
                uuid,
                &element,
                "change",
                Box::new(move |_, e: Event| {
                    let res = e.into_serde::<Detail<Value>>().unwrap();
                    // let val = res._detail._value.parse::<i32>().unwrap();
                    onchange.emit(res._detail._value);
                }),
            );

            let this_link = link.to_owned();
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
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This {
            props,
            node,
            data,
            uuid,
            ..
        } = &this;
        let width = props.width.to_string();
        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "width": width,
                        "--mdc-theme-secondary": theme.slider.color,
                    }
                })
            }),
            "Slider",
        );

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                    .mdc-slider__track-container{{
                        background-color: {};
                    }}
                    "#,
                    theme.slider.background,
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        html! {
            <mwc-slider
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("{} {}", props.class, style.theme_item("jss"))
                slot?=props.slot.as_ref()
                value?=props.value.as_ref()
                min?=props.min.as_ref()
                max?=props.max.as_ref()
                step?=props.step.as_ref()
                pin?=to_option(props.pin)
                markers?=to_option(props.markers)
                disabled=props.disabled
            />
        }
    },
    "./doc/slider.md"
);
