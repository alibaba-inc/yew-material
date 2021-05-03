use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgImg {}

impl Component for PgImg {
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
                <div class=style("title")>{"Img 图片"}</div>
                <div>{"图片组件拥有多种图片展示形式"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"加载动效：拥有网络图片在加载过程中的动画效果"}</li>
                    <li>{"背景图片：当组件有子元素时将自动转换为背景图"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"img\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::img::Img;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex direction="column">
                        <Img margin="20px 0" width="120px" height="120px" src="http://error" />
                        <Img width="250px" radius="25%" src="https://ss0.bdstatic.com/70cFuHSh_Q1YnxGkpoWK1HF6hhy/it/u=2498791300,2171078002&fm=26&gp=0.jpg" />
                        <Img
                            src="https://ss0.bdstatic.com/70cFuHSh_Q1YnxGkpoWK1HF6hhy/it/u=2498791300,2171078002&fm=26&gp=0.jpg"
                            margin="20px 0"
                            padding="15px 20px"
                            size="cover"
                            position="bottom"
                        >
                            <Text color="#fff">
                                <p>{"Love is more than a word，it says so much."}</p>
                                <p>{" When I see these four letters, I almost feel your touch."}</p>
                            </Text>
                        </Img>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Img width=\"120px\" height=\"120px\" src=\"http://error\" />
<Img width=\"250px\" radius=\"25%\" src=\"https://ss0.bdstatic.com/...\" />
<Img margin=\"20px 0\" padding=\"15px 20px\" size=\"cover\" position=\"bottom\" src=\"https://ss0.bdstatic.com/...\">
    <Text color=\"#fff\">
        <p>{\"Love is more than a word，it says so much.\"}</p>
        <p>{\" When I see these four letters, I almost feel your touch.\"}</p>
    </Text>
</Img>
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("src", "图片地址", "String", "", "")}
                        {table_th("width", "图片宽度", "String", "", "")}
                        {table_th("height", "图片高度", "String", "", "")}
                        {table_th("margin", "图片外边距", "String", "", "")}
                        {table_th("padding", "图片内边距，只在组件有子元素时生效", "String", "", "")}
                        {table_th("radius", "图片边框圆角，取值同css border-radius", "String", "", "")}
                        {table_th("size", "背景图片大小，只在组件有子元素时生效，取值同css background-size", "String", "", "")}
                        {table_th("repeat", "背景图片重复模式，只在组件有子元素时生效，取值同css background-repeat", "String", "", "")}
                        {table_th("position", "背景图片位置，只在组件有子元素时生效，取值同css background-position", "String", "", "")}
                    </Flex>
                </div>
                {switch_bottom("iframe", "Iframe 内嵌子框架", "innerhtml", "InnerHtml")}
            </Animate>
        }
    }
}
