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
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| false)]
    pub center_title: bool,
    #[prop_or_else(|| false)]
    pub dense: bool,
    #[prop_or_else(|| false)]
    pub prominent: bool,
    #[prop_or_else(|| false)]
    pub fixed: bool,
    #[prop_or_else(|| 4)]
    pub index: u32,
    #[prop_or_else(|| "100%".into())]
    width: String,
}

pub type This = Appbar;

comp_theme!(
    Appbar,
    |this: &This, first: bool| {
        if first {
            imports("appbar");
            let This { link, uuid, .. } = &this;
            reload::<This>(uuid, link);
        }
    },
    |uuid: &String, _| destroy(&uuid),
    |this: &This| {
        let This { props, .. } = &this;
        let pop = props.to_owned();
        let title = to_option(props.center_title);
        let dense = to_option(props.dense);
        let prominent = to_option(props.prominent);
        let height = match props.dense {
            true if !props.prominent => 48,
            true if props.prominent => 96,
            false if props.prominent => 128,
            _ => 64,
        };

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "width": pop.width,
                        "position": match pop.fixed {
                            true => "fixed",
                            _ => "relative",
                        },
                        "z-index": pop.index,
                        "--mdc-theme-primary": theme.appbar.background,
                        "--mdc-top-app-bar-width": pop.width,
                        "--mdc-typography-headline6-font-family": theme.font_family,
                        "--mdc-top-app-bar-fixed-box-shadow": "none",
                        "box-shadow": match pop.fixed {
                            true => &theme.appbar.shadow,
                            _ => "none",
                        },
                    },
                    "holder": {
                        "display": "table",
                        "width": pop.width,
                        "height": format!("{}px", height),
                    },
                    "@media (max-width: 600px)": {
                        "holder": {
                            "height": format!("{}px", height - 8),
                        }
                    }
                })
            }),
            "Appbar",
        );

        let children = props.children.to_owned();
        let id = props.id.as_ref();
        let class = format!("{} {}", props.class, style.theme_item("jss"));

        match props.fixed {
            true => html! {
                <>
                    <mwc-top-app-bar-fixed
                        id?=id
                        class=class
                        centerTitle?=title
                        dense?=dense
                        prominent?=prominent
                    >
                        {children}
                    </mwc-top-app-bar-fixed>
                    <div class=format!("mwc_app_bar_holder {}", style.item("holder")) />
                </>
            },
            _ => html! {
                <mwc-top-app-bar
                    id?=id
                    class=class
                    centerTitle?=title
                    dense?=dense
                    prominent?=prominent
                >
                    {children}
                </mwc-top-app-bar>
            },
        }
    },
    "./doc/appbar.md"
);
