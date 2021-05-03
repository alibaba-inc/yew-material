use yew::prelude::*;
use yew_material::prelude::*;
use yew_material_utils::prelude::*;

use crate::route::*;
use crate::styles::*;
use crate::theme::Extra;

pub struct PgStyle {}

impl Component for PgStyle {
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
        let themes = to_theme::<Extra>();
        let border_color = match themes.ident.as_str() {
            "light" => "#5da913",
            "dark" => "#0082c2",
            _ => "#000",
        };
        let style_easy = Style::easy(json!({
            "case": {
                "width": "200px",
                "text-align": "right",
                "padding": "15px 30px",
                "border-color": border_color,
                "border": "solid 2px",
                "& p": {
                    "line-height": "24px",
                    "color": "#0e75f3",
                },
                "& .list": {
                    "color": "#5da913"
                },
            },
            "case_text": {
                "font-size": "18px",
                "font-weight": "bold",
            },
            "@media (max-width: 600px)": {
                "case": {
                    "text-align": "left!important",
                    "& p.list": {
                        "color": "#ff0000"
                    },
                },
            },
        }));
        let style_theme = create_style::<Extra>(Box::new(|theme: &Theme<Extra>| {
            json!({
                "box": {
                    "background": theme.extra.footer_background,
                }
            })
        }));
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Style 样式"}</div>
                <div>
                    <Text>{"本框架样式部分采用Jss方案，详细资料可查看："}</Text>
                    <Text color="#0e75f3"><a target="_blank" href="https://cssinjs.org">{"Jss官网"}</a></Text>
                </div>
                {static_des(1)}
                {link_des("#")}
                {code("
//方法引入
use yew_material_utils::prelude::*;
//或者
use yew_material_utils::{
    json,
    style::{create_style, Item, Style},
    theme::{to_theme, Theme},
};
                ")}
                <div class=style("subtitle")>{"方法展示"}</div>
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <div class=format!("{} {}", style_easy.item("case"), style_theme.item("box"))>
                        <Text class=style_easy.item("case_text")>{"示例文本"}</Text>
                        <p>{"这是p标签"}</p>
                        <p class="list">{"拥有list样式名"}</p>
                        <p class="list">{"拥有list样式名"}</p>
                    </div>
                </Flex>
                <div>
                    {code("
fn view(&self) -> Html {

    //Extra扩展结构体详见Theme文档
    let themes = to_theme::<Extra>();
    //此处只是为了演示，不同主题配色需求应使用下方的Extra扩展方案
    let border_color = match themes.ident.as_str() {
        \"light\" => \"#5da913\",
        \"dark\" => \"#0082c2\",
        _ => \"#000\",
    };

    //简易用法
    let style = Style::easy(json!({
        \"case\": {
            \"width\": \"200px\",
            \"text-align\": \"right\",
            \"padding\": \"15px 30px\",
            \"border-color\": border_color,
            \"border\": \"solid 2px\",
            \"& p\": {
                \"line-height\": \"24px\",
                \"color\": \"#0e75f3\",
            },
            \"& .list\": {
                \"color\": \"#5da913\"
            },
        },
        \"case_text\": {
            \"font-size\": \"18px\",
            \"font-weight\": \"bold\",
        },
        \"@media (max-width: 600px)\": {
            \"case\": {
                \"text-align\": \"left!important\",
                \"& p.list\": {
                    \"color\": \"#ff0000\"
                },
            },
        },
    }));

    //扩展用法
    let style_theme = create_style::<Extra>(Box::new(|theme: &Theme<Extra>| {
        json!({
            \"box\": {
                \"background\": theme.extra.footer_background,
            }
        })
    }));

    html! {
        <div class=format!(\"{} {}\", style_easy.item(\"case\"), style_theme.item(\"box\"))>
            <Text class=style.item(\"case_text\")>{\"示例文本\"}</Text>
            <p>{\"这是p标签\"}</p>
            <p class=\"list\">{\"拥有list样式名\"}</p>
            <p class=\"list\">{\"拥有list样式名\"}</p>
        </div>
    }
}
                    ")}
                </div>
                {switch_bottom("usage", "使用", "theme", "Theme 主题")}
            </Animate>
        }
    }
}
