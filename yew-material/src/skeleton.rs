use crate::{comp_theme, flex::Flex, styles::*};
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
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub slot: String,
    #[prop_or_default]
    pub width: String,
    #[prop_or_else(|| "18px".into())]
    pub item_height: String,
    #[prop_or_default]
    pub item_radius: String,
    #[prop_or_else(|| "0px".into())]
    pub margin: String,
    #[prop_or_else(|| vec!["38%","100%","100%","61%"])]
    pub rows: Vec<&'static str>,
    #[prop_or_default]
    pub item_align: String,
    #[prop_or_else(|| true)]
    pub loading: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
}

pub type This = Skeleton;

comp_theme!(
    Skeleton,
    |this: &This, first: bool| {
        let This {
            props, link, uuid, ..
        } = &this;
        if first {
            if props.auto_theme {
                reload::<This>(uuid, link);
            }
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, .. } = &this;
        let pop = props.to_owned();

        let grow = match props.width.as_str() {
            "" => 1,
            _ => 0,
        };

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "& div:last-child": {
                            "margin": "0px"
                        }
                    },
                    "item": {
                        "display": "block",
                        "height": pop.item_height,
                        "margin": "0 0 15px 0",
                        "border-radius": pop.item_radius,
                        "background": theme.skeleton.background,
                        "background-size": "400% 100%",
                        "animation-iteration-count": "infinite",
                    },
                })
            }),
            "Skeleton",
        );

        match props.loading {
            true => html! {
                <Flex
                    id=props.id.to_string()
                    class=format!("{} {}", props.class, style.theme_item("jss"))
                    slot=props.slot.to_string()
                    width=props.width.to_string()
                    margin=props.margin.to_string()
                    align=props.item_align.to_string()
                    direction="column"
                    grow=grow
                >
                    {
                        for props.rows.iter().map(|width| html!{
                            <div
                                class=format!("{} {}", style.item("item"), transition_bg())
                                style=format!("width: {}", width)
                            />
                        })
                    }
                </Flex>
            },
            _ => html! { <> {props.children.to_owned()} </>},
        }
    },
    "./doc/skeleton.md"
);
