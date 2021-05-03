use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgSkeleton {}

impl Component for PgSkeleton {
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
                <div class=style("title")>{"Skeleton 骨架屏"}</div>
                <div>{"在内容需要等待渲染的位置提供一组加载动效"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"skeleton\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::skeleton::Skeleton;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex direction="column" padding="15px 0">
                        <Skeleton />
                        <Skeleton loading=false>
                            <Text padding="20px 0">{"加载完成..."}</Text>
                        </Skeleton>
                        <Flex>
                            <Skeleton width="100px" item_height="100px" rows=vec!["100%"] item_radius="50%" />
                            <Skeleton item_height="24px" rows=vec!["35%","26%","45%"] item_radius="4px" item_align="flex-end" />
                        </Flex>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                {code("
<Skeleton />
<Skeleton loading=false>
    <Text padding=\"20px 0\">{\"加载完成...\"}</Text>
</Skeleton>
<Flex>
    <Skeleton width=\"100px\" item_height=\"100px\" rows=vec![\"100%\"] item_radius=\"50%\" />
    <Skeleton item_height=\"24px\" rows=vec![\"35%\",\"26%\",\"45%\"] item_radius=\"4px\" item_align=\"flex-end\" />
</Flex>
                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    {notice_des("loading=false时，Skeleton组件只会渲染children内容，即width、margin、item_align等属性不会参与children内容的排版布局")}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("width", "骨架屏外宽度", "String", "", "")}
                        {table_th("margin", "骨架屏外间距", "String", "", "")}
                        {table_th("loading", "是否开启骨架屏加载动效", "bool", "true", "")}
                        {table_th("rows", "子骨架行数，包含单个宽度", "Vec<str>", "", "")}
                        {table_th("item_height", "子骨架高度", "String", "18px", "")}
                        {table_th("item_radius", "子骨架圆角", "String", "", "")}
                        {table_th("item_align", "子骨架水平对齐方式，取值同css align-items", "String", "", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("background", "骨架屏背景动效")}
                    </Flex>
                </div>
                {switch_bottom("select", "Select 选择器", "slider", "Slider 滑动条")}
            </Animate>
        }
    }
}
