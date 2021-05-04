use crate::theme::Extra;
use yew::prelude::*;
use yew_material_utils::{
    json,
    style::{create_style, Item},
    theme::Theme,
};

pub fn style(item: &str) -> String {
    create_style::<Extra>(Box::new(|theme: &Theme<Extra>| {
        let code = &theme.extra.code;
        json!({
            "body": {
                "position": "absolute",
                "width": "100%!important",
                "height": "100%!important",
                "& .left_meun": {
                    "top": "64px"
                },
            },
            "version": {
                "padding": "0 10px",
                "font-size": "12px",
            },
            "title":  {
                "padding": "10px 0",
                "font-size": "30px",
            },
            "subtitle":  {
                "padding": "40px 0 10px 0",
                "font-size": "24px",
            },
            "describe":  {
                "margin-top": 0,
                "line-height": "24px",
            },
            "exhibition":  {
                "padding": "10px 0",
                "margin-top": "10px",
            },
            "space": {
                "margin-left": "10px",
            },
            "container": {
                "padding": "40px 34px 10px 34px",
            },
            "exhibition": {
                "display": "inline-block!important",
                "box-sizing": "border-box!important",
                "width": "100%!important",
                "margin-top": "10px",
                "padding": "5px 0!important",
                "& a": {
                    "color": "#0e75f3",
                },
                "& .item": {
                    "margin": "5px 10px 5px 0",
                    "vertical-align": "bottom",
                },
                "& .exp_animate": {
                    "background": "#b6b808",
                    "color": "#042bec",
                },
            },
            "code": {
                "margin": "10px 0",
                "line-height": "20px",
                "border-left": "3px solid #5da913",
                "color": code.color,
                "background": code.background,
                "box-shadow": code.shadow,
                "& pre": {
                    "display": "table",
                    "padding": "0 20px",
                    "margin": "0",
                    "white-space": "break-spaces",
                    "word-break": "break-all",
                    "font-family": "revert",
                    "font-size": "14px",
                }
            },
            "link": {
                "& a": {
                    "color": "#0e75f3",
                    "cursor": "pointer",
                    "transition": "color 0.3s",
                    "text-decoration": "none",
                },
                "& a:hover": {
                    "color": "#045cc7",
                }
            },
            "footer": {
                "padding": "0 34px 30px 34px",
                "background": theme.extra.footer_background,
                "& .footer_items a": {
                    "display": "block",
                    "color": "inherit",
                    "font-size": "14px",
                    "margin": "5px 0",
                    "transition": "color 0.3s",
                },
                "& .footer_items a:hover": {
                    "color": "#1890FF",
                }
            },
            "@media (max-width: 600px)": {
                "body": {
                    "& .env_mobile_hide": {
                        "display": "none",
                    },
                    "& .left_meun": {
                        "top": "56px!important",
                    },
                    "& .footer_box": {
                        "margin": "0px!important"
                    }
                },
                "container": {
                    "padding": "30px 24px 10px 24px!important",
                },
            },
        })
    }))
    .item(item)
}

pub fn code(text: &str) -> Html {
    html! {
        <div class=style("code")><pre>{text}</pre></div>
    }
}
