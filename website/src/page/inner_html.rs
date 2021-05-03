use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgInnerHtml {}

impl Component for PgInnerHtml {
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
        let html = r#"
            <div>
                <p style="text-align: center;">
                    <img src="https://www.w3cschool.cn/attachments/image/20170526/1495798408306247.jpg" />
                    <br>
                </p>
                <h2>HTML发展史</h2>
                <p style="line-height:26px;color:#6d6bff;">HTML没有1.0，因为关于它的初版存在争议，1995年HTML 2.0面世，1997年由国际官方组织W3C推出了HTML 3.2以及HTML 4.0标准，后面W3C(万维网联盟)也渐渐变成Web技术领域的权威，经过漫长的演变，2014年，HTML 5标准最终面世。</p>
            </div>"#;
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"InnerHtml"}</div>
                <div>{"将html文本转换为虚拟节点"}</div>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"innerhtml\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::innerhtml::InnerHtml;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="25px">
                    <InnerHtml html=html />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
fn view(&self) -> Html {
    let html = r#\"
        <div>
            <p style=\"text-align: center;\">
                <img src=\"https://www.w3cschool.cn/attachments/image/20170526/1495798408306247.jpg\" />
                <br>
            </p>
            <h2>HTML发展史</h2>
            <p style=\"line-height:26px;color:#6d6bff;\">HTML没有1.0，因为关于它的初版存在争议，1995年HTML 2.0面世，1997年由国际官方组织W3C推出了HTML 3.2以及HTML 4.0标准，后面W3C(万维网联盟)也渐渐变成Web技术领域的权威，经过漫长的演变，2014年，HTML 5标准最终面世。</p>
        </div>\"#;
    html! {
        <InnerHtml html=html />
    }
}
                ")}
                    {static_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("html", "html文本内容", "String", "", "")}
                    </Flex>
                </div>
                {switch_bottom("img", "Img 图片", "list", "List 列表")}
            </Animate>
        }
    }
}
