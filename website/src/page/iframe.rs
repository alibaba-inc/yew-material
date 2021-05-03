use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgIframe {}

impl Component for PgIframe {
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
                <div class=style("title")>{"Iframe 内嵌子框架"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"自适应宽高：子框架宽高可根据其内容进行改变"}</li>
                    <li>{"主题互通：子框架主题可跟随主框架主题改变"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"iframe\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::iframe::Iframe;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex align="center" margin="15px 0 10px 0">
                        <Text>{"Iframe内容"}</Text>
                        <Text color="#0e75f3" padding="0 10px"><a target="_blank" href="/?exp=iframe">{"open new window"}</a></Text>
                    </Flex>
                    <Iframe src="/?exp=iframe" />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Iframe src=\"xxx\" />
                    ")}
                    {static_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("src", "子框架服务地址", "String", "", "")}
                        {table_th("width", "宽度", "String", "100%", "")}
                        {table_th("height", "高度，不设置则为自适应", "String", "", "")}
                    </Flex>
                </div>
                {switch_bottom("iconbutton", "IconButton 图标按钮", "img", "Img 图片")}
            </Animate>
        }
    }
}
