use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Change(i32),
}

pub struct PgSlider {
    link: ComponentLink<Self>,
    value: i32,
}

impl Component for PgSlider {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(val) => {
                self.value = val;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Slider 滑动条"}</div>
                <div>{"用于在多个选项中选中某个选项"}</div>
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
    \"slider\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::slider::Slider;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex padding="0 20px" direction="column">
                        <Text align="center" padding="20px 0 0 0">{"onchange value："}{self.value}</Text>
                        <Slider
                            class="item"
                            width="100%"
                            max=200
                            onchange=&self.link.callback(|val| Msg::Change(val))
                        />
                        <Slider
                            class="item"
                            width="100%"
                            max=200
                            step=20
                            pin=true
                            markers=true
                            onchange=&self.link.callback(|val| Msg::Change(val))
                        />
                        <Slider class="item" width="100%" disabled=true />
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Change(i32),
}

pub struct Page {
    link: ComponentLink<Self>,
    value: i32,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(val) => {
                self.value = val;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Text align=\"center\">{\"onchange value：\"}{self.value}</Text>
                <Slider
                    width=\"100%\"
                    max=200
                    onchange=&self.link.callback(|val| Msg::Change(val))
                />
                <Slider
                    width=\"100%\"
                    max=200
                    step=20
                    pin=true
                    markers=true
                    onchange=&self.link.callback(|val| Msg::Change(val))
                />
                <Slider width=\"100%\" disabled=true />
            </>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("value", "组件值", "i32", "0", "")}
                        {table_th("width", "组件宽度", "String", "", "")}
                        {table_th("min", "最小值", "i32", "0", "")}
                        {table_th("max", "最大值", "i32", "100", "")}
                        {table_th("step", "滑动间隔", "i32", "0", "")}
                        {table_th("pin", "是否显示滑块拇指针", "bool", "false", "")}
                        {table_th("markers", "是否为离散滑块", "bool", "false", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("onchange", "值变更后回调函数", "Callback", "", "")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "已滑动部分颜色")}
                        {other_th("background", "待滑动部分背景色")}
                    </Flex>
                </div>
            {switch_bottom("skeleton", "Skeleton 骨架屏", "snackbar", "Snackbar 提醒框")}
            </Animate>
        }
    }
}
