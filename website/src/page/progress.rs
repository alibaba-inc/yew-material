use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgProgress {}

impl Component for PgProgress {
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
                <div class=style("title")>{"Progress 进度条"}</div>
                <div>{"常用于展示操作的当前进度"}</div>
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
    \"progress\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::progress::Progress;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex align="center" padding="10px 0 0 0">
                        <Progress progress=0.7 color="#ff0000" />
                        <Progress indeterminate=true size="28px" margin="0 25px" />
                        <Progress indeterminate=true density=-5 />
                    </Flex>
                    <Divider />
                    <Flex direction="column" padding="0 0 15px 0">
                        <Progress r#type="linear" width="40%" progress=0.7 color="#ff0000" />
                        <Progress r#type="linear" width="80%" progress=0.3 buffer=0.6 margin="15px 0" />
                        <Progress r#type="linear" width="60%" indeterminate=true margin="0 0 15px 0" />
                        <Progress r#type="linear" indeterminate=true reverse=true />
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Flex align=\"center\" padding=\"10px 0 0 0\">
    <Progress progress=0.7 color=\"#ff0000\" />
    <Progress indeterminate=true size=\"28px\" margin=\"0 25px\" />
    <Progress indeterminate=true density=-5 />
</Flex>
<Divider />
<Flex direction=\"column\" padding=\"0 0 15px 0\">
    <Progress r#type=\"linear\" width=\"40%\" progress=0.7 color=\"#ff0000\" />
    <Progress r#type=\"linear\" width=\"80%\" progress=0.3 buffer=0.6 margin=\"15px 0\" />
    <Progress r#type=\"linear\" width=\"60%\" indeterminate=true margin=\"0 0 15px 0\" />
    <Progress r#type=\"linear\" indeterminate=true reverse=true />
</Flex>
                ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("r#type", "类型，circular：环形，linear：线性", "String", "circular", "")}
                        {table_th("color", "颜色", "String", "", "")}
                        {table_th("progress", "进度，取值范围：0.0 ~ 1.0", "f64", "0.0", "")}
                        {table_th("indeterminate", "开启随机进度", "bool", "false", "")}
                        {table_th("margin", "外边距", "String", "", "")}
                        {table_th("size", "尺寸，依照宽高来改变尺寸，仅在r#type:circular下生效", "String", "", "")}
                        {table_th("density", "密度，依照密度来改变尺寸，最小值为-8，仅在r#type:circular下生效", "i32", "0", "")}
                        {table_th("width", "宽度，仅在r#type:linear下生效", "String", "100%", "")}
                        {table_th("buffer", "缓冲区，取值范围：0.0 ~ 1.0，仅在r#type:linear下生效", "f64", "1.0", "")}
                        {table_th("reverse", "进度条翻转，仅在r#type:linear下生效", "bool", "false", "")}
                        {table_th("closed", "关闭", "bool", "false", "")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "整体颜色")}
                        {other_th("linear_buffer", "缓冲区颜色")}
                        {other_th("linear_buffer_dots", "缓冲区圆点颜色")}
                    </Flex>
                </div>
                {switch_bottom("menu", "Menu 下拉菜单", "radio", "Radio 单选框")}
            </Animate>
        }
    }
}
