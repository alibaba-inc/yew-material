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
                // "background": "#303030",
                // "body": {
                //     "background": "#303030",
                // },
                "extra": {
                    "code": {
                        "color": "#fff",
                        "background": "#3f3f3f",
                        "shadow": "0 0 3px 1px #222",
                    },
                    "footer_background": "#1c1c1c",
                }
            }
        }),
    );
}
