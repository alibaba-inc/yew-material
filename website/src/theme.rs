use yew_material_utils::{
    json,
    theme::{get_theme_ident, init},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct Extra {
    pub code: CodeCss,
    pub footer_background: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CodeCss {
    pub color: String,
    pub background: String,
    pub shadow: String,
}

pub fn initialize() {
    let mut ident = get_theme_ident(true);
    if ident == "" {
        ident = "dark".into();
    }
    init(
        ident,
        json!({
            "light": {
                "extra": {
                    "code": {
                        "color": "#000",
                        "background": "#fff",
                        "shadow": "0 0 3px 1px #aeacac",
                    },
                    "footer_background": "#eee",
                }
            },
            "dark": {
                "extra": {
                    "code": {
                        "color": "#fff",
                        "background": "#3f3f3f",
                        "shadow": "0 0 3px 1px #222",
                    },
                    "footer_background": "#1c1c1c",
                }
            },
            "custom": {
                "font_family": "SimSun",
                // "font_weight": 300,
                // "color": "rgba(255, 255, 255, 0.92)",
                "background": "#0a112b",
                // "ripple": "rgba(255, 255, 255, 0.18)",
                // "hover": "rgba(255, 255, 255, 0.06)",
                // "checked": "#009906",
                // "unchecked": "rgba(255, 255, 255, 0.54)",
                // "activated": "#a9a3ff",
                // "selected": "rgba(255, 255, 255, 0.12)",
                // "disabled": "rgba(255, 255, 255, 0.18)",
                // "icon_color": "#73920c",
                // "body": {
                //     "color": "",
                //     "background": "",
                // },
                // "divider": {
                //     "color": "",
                // },
                // "skeleton": {
                //     "background": "",
                // },
                // "progress": {
                //     "color": "",
                //     "linear_buffer": "",
                //     "linear_buffer_dots": "",
                // },
                // "scrollbar": {
                //     "background":"",
                // },
                // "button": {
                //     "color": "",
                //     "text_button_color": "",
                //     "background": "",
                //     "ripple": "",
                //     "disabled_color": "",
                //     "disabled_background": "",
                // },
                "appbar": {
                    // "shadow": "",
                    "background": "#11173d",
                },
                // "icon": {
                //     "color": "",
                // },
                // "radio": {
                //     "checked": "",
                //     "unchecked": "",
                //     "disabled": "",
                // },
                // "checkbox": {
                //     "checked": "",
                //     "unchecked": "",
                //     "disabled": "",
                // },
                // "switch": {
                //     "checked":"",
                //     "unchecked":"",
                //     "disabled":"",
                // },
                // "slider": {
                //     "color": "",
                //     "background": "",
                // },
                // "list": {
                //     "background": "",
                //     "ripple": "",
                //     "selected": "",
                //     "hover": "",
                // },
                // "list_item": {
                //     "secondary_color": "",
                // },
                // "tab_item": {
                //     "color": "",
                //     "icon_color": "",
                //     "activated": "",
                //     "ripple": "",
                // },
                // "dialog": {
                //     "heading_color": "",
                //     "content_color": "",
                //     "background":"",
                // },
                "textfield": {
                    // "color": "",
                    "background": "#1b1950",
                    // "error_color": "",
                    // "label_color": "",
                    // "label_focus_color": "",
                    // "icon_color": "",
                    // "disabled_color": "",
                    "disabled_background": "#38394a",
                    "radius": "0px",
                    // "border_color": "",
                    // "border_hover_color": "",
                    // "border_disabled_color": "",
                },
                // "snackbar": {
                //     "color": "",
                //     "background": "",
                // },
                "extra": {
                    "code": {
                        "color": "#fff",
                        "background": "#001c34",
                        "shadow": "0 0 3px 1px #0f0039",
                    },
                    "footer_background": "#000231",
                }
            },
        }),
    );
}
