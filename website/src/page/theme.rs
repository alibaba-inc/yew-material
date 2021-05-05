use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgTheme {}

impl Component for PgTheme {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Theme 主题"}</div>
                {link_des("#")}
                {static_des(1)}
                <div>
                    <Text>{"自定义主题和完整配置项可查看此文件："}</Text>
                    <Text color="#0e75f3">
                        <a target="_blank" href="https://github.com/alibaba-inc/yew-material/blob/master/website/src/theme.rs">
                            {"Theme配置"}
                        </a>
                    </Text>
                </div>
                {code("
use yew_material_utils::prelude::*;
//或者
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
    if ident == \"\" {
        ident = \"dark\".into();
    }
    init(
        ident,
        json!({
            \"light\": {
                \"button\": {
                    \"background\": \"#2b2b2b\",
                },
                \"extra\": {
                    \"code\": {
                        \"color\": \"#000\",
                        \"background\": \"#fff\",
                        \"shadow\": \"0 0 3px 1px #aeacac\",
                    },
                    \"footer_background\": \"#eee\",
                }
            },
            \"dark\": {
                \"body\": {
                    \"background\": \"#303030\",
                },
                \"extra\": {
                    \"code\": {
                        \"color\": \"#fff\",
                        \"background\": \"#3f3f3f\",
                        \"shadow\": \"0 0 3px 1px #222\",
                    },
                    \"footer_background\": \"#1c1c1c\",
                }
            }
        }),
    );
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    initialize();

    let root = document().get_element_by_id(\"root\").unwrap();
    App::<Index>::new().mount(root);

    Ok(())
}
                ")}
                <Divider margin="15px 0 35px 0" />
                <div class=style("title")>{"Ident 标识"}</div>
                <Flex direction="column" border_top=true>
                    {other_th("属性", "说明")}
                    {other_th("light", "亮色主题")}
                    {other_th("dark", "暗色主题")}
                </Flex>
                {switch_bottom("style", "Style 样式", "animate", "Animate 动画")}
            </Animate>
        }
    }
}
