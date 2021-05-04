use crate::icon::index::*;
use crate::progress::Progress;
use crate::{comp_theme, imports, styles::Extra, to_option};
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::{to_theme, Theme},
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| "button".into())]
    pub r#type: String,
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub background: String,
    #[prop_or_default]
    pub ripple: String,
    #[prop_or_else(|| false)]
    pub unelevated: bool,
    #[prop_or_default]
    pub border_width: String,
    #[prop_or_default]
    pub border_color: String,
    #[prop_or_else(|| false)]
    pub dense: bool,
    #[prop_or_else(|| false)]
    pub trailing_icon: bool,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub full_width: bool,
    #[prop_or_else(|| false)]
    pub text_button: bool,
    #[prop_or_else(|| None)]
    pub dialog_action: Option<String>,
    #[prop_or_else(|| false)]
    pub loading: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub type This = Button;

comp_theme!(
    Button,
    |_: &This, first: bool| {
        if first {
            imports("button");
        }
    },
    |_: &String, _: &NodeRef| {},
    |this: &This| {
        let This { props, .. } = &this;
        let Props {
            children,
            full_width,
            loading,
            onclick,
            ..
        } = props.to_owned();
        let pop = props.to_owned();

        let raised = match props.text_button {
            _ if props.unelevated => false,
            true => false,
            _ => true,
        };

        let outlined = match props.border_width.as_str() {
            "" => false,
            _ => true,
        };

        let theme = to_theme::<Extra>();
        let color = match props.color.as_str() {
            _ if props.disabled => &theme.button.disabled_color,
            x if props.text_button => match x {
                "" => &theme.button.text_button_color,
                _ => x,
            },
            "" => &theme.button.color,
            _ => &props.color,
        };
        let btn_color = color.to_owned();

        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                let background = match pop.background.as_str() {
                    _ if pop.text_button => &btn_color,
                    "" if outlined => "",
                    "" => &theme.button.background,
                    _ => &pop.background,
                };
                json!({
                    "jss": {
                        "display": "inline-flex",
                        "border": "none",
                        "background": "none",
                        "outline": "none",
                        "padding": 0,
                        "width": if full_width { "100%" } else { "auto" },
                    },
                    "button": {
                        "--mdc-button-disabled-ink-color": btn_color,
                        "--mdc-button-disabled-fill-color": theme.button.disabled_background,
                        "--mdc-button-outline-width": pop.border_width,
                        "--mdc-button-outline-color": pop.border_color,
                        "--mdc-theme-on-primary": btn_color,
                        "--mdc-typography-button-font-family": theme.font_family,
                        "--mdc-typography-button-font-weight": match pop.text_button {
                            true => 500,
                            _ => 300
                        },
                        "--mdc-typography-button-font-size": match pop.dense {
                            true => "12px",
                            _ => "14px",
                        },
                        "background": match outlined {
                            true => background,
                            _ => "",
                        },
                        "--mdc-theme-primary": match outlined {
                            true => &btn_color,
                            _ => background,
                        },
                        "--mdc-ripple-color": match pop.ripple.as_str() {
                            _ if pop.text_button => &theme.ripple,
                            "" => &theme.button.ripple,
                            _ => &pop.ripple,
                        },
                    },
                    "icon": {
                        "height": "18px",
                        "line-height": match pop.dense {
                            true => "19px",
                            _ => "20px",
                        },
                    }
                })
            }),
            "Button",
        );

        let cpm_icon = |margin| html! {<Icon class=style.item("icon") color=color size="18px" margin=margin icon=&props.icon />};
        let cmp_progress =
            |margin| html! {<Progress indeterminate=true color=color size="20px" margin=margin />};
        html! {
            <button
                slot?=props.slot.as_ref()
                class=style.item("jss")
                type=if loading {"button"} else {props.r#type.as_str()}
            >
                <mwc-button
                    id?=props.id.as_ref()
                    class=format!("{} {}", props.class, style.theme_item("button"))
                    raised?=to_option(raised)
                    unelevated?=to_option(props.unelevated)
                    outlined?=to_option(outlined)
                    dense?=to_option(props.dense)
                    disabled=props.disabled
                    fullWidth?=to_option(props.full_width)
                    dialogAction?=props.dialog_action.as_ref()
                    onclick=Callback::from(move |e| {
                        if !loading {onclick.emit(e);}
                    })
                >
                    {
                        match props.trailing_icon {
                            false if loading => cmp_progress("0 8px 0 0"),
                            false if props.icon != "" => cpm_icon("0 8px 0 0"),
                            _ => html!{<></>},
                        }
                    }
                    {&props.label}
                    {children}
                    {
                        match props.trailing_icon {
                            true if loading => cmp_progress("0 0 0 7px"),
                            true if props.icon != "" => cpm_icon("0 0 0 7px"),
                            _ => html!{<></>},
                        }
                    }
                </mwc-button>
            </button>
        }
    },
    "./doc/button.md"
);

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use yew_material_utils::test::run;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    async fn can_create_btn() {
        let html = html! {
            <Button dense=true icon="code">{"Button-children"}</Button>
        };
        let (html, _node) = run("Button-box", html).await;
        assert_eq!(
            html.contains("mwc-button dense=\"\" raised=\"\" "),
            true,
            "Button Props error"
        );
        assert_eq!(
            html.contains("Button-children"),
            true,
            "Button children error"
        );
    }
}
