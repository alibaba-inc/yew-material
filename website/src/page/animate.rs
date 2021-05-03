use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Set(&'static str, i32),
}

pub struct PgAnimate {
    link: ComponentLink<Self>,
    r#type: &'static str,
    width: i32,
    height: i32,
    top: i32,
    left: i32,
    deg: i32,
    time: f32,
}

impl Component for PgAnimate {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            r#type: "width",
            width: 100,
            height: 100,
            top: 0,
            left: 0,
            deg: 0,
            time: 0.3,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(ty, n) => {
                match ty {
                    "width" => self.width = self.width + n,
                    "height" => self.height = self.height + n,
                    "top" => self.top = self.top + n,
                    "left" => self.left = self.left + n,
                    "rotate" => self.deg = self.deg + n,
                    _ => (),
                };
                self.time = match ty {
                    "opacity" => 3.0,
                    _ => 0.3,
                };
                self.r#type = ty;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let n = 50;
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Animate 动画"}</div>
                <div>{"可以快速实现各种简易css动画效果"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"多种动效：内置多种常见动效"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"animate\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::animate::Animate;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Button class="item" dense=true label="宽度加" onclick=&self.link.callback(move |_| Msg::Set("width", n)) />
                    <Button class="item" dense=true label="宽度减" onclick=&self.link.callback(move |_| Msg::Set("width", -n)) />
                    <Button class="item" dense=true label="高度加" onclick=&self.link.callback(move |_| Msg::Set("height", n)) />
                    <Button class="item" dense=true label="高度减" onclick=&self.link.callback(move |_| Msg::Set("height", -n)) />
                    <Button class="item" dense=true label="距上加" onclick=&self.link.callback(move |_| Msg::Set("top", n)) />
                    <Button class="item" dense=true label="距上减" onclick=&self.link.callback(move |_| Msg::Set("top", -n)) />
                    <Button class="item" dense=true label="距左加" onclick=&self.link.callback(move |_| Msg::Set("left", n)) />
                    <Button class="item" dense=true label="距左减" onclick=&self.link.callback(move |_| Msg::Set("left", -n)) />
                    <Button class="item" dense=true label="顺时针转" onclick=&self.link.callback(move |_| Msg::Set("rotate", n)) />
                    <Button class="item" dense=true label="逆时针转" onclick=&self.link.callback(move |_| Msg::Set("rotate", -n)) />
                    <Button class="item" dense=true label="透明度渐变" onclick=&self.link.callback(move |_| Msg::Set("opacity", 0)) />
                    <Flex padding="15px 0">
                        <Animate
                            r#type=self.r#type
                            width=format!("{}px", self.width)
                            height=format!("{}px", self.height)
                            position="relative"
                            top=format!("{}px", self.top)
                            left=format!("{}px", self.left)
                            deg=self.deg
                            time=self.time
                            every_time=true
                            index=9
                        >
                            <Flex class="exp_animate" width="100%" height="100%" border=true justify="center" align="center">{"动画对象"}</Flex>
                        </Animate>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Set(&'static str, i32),
}

pub struct PgAnimate {
    link: ComponentLink<Self>,
    r#type: &'static str,
    width: i32,
    height: i32,
    top: i32,
    left: i32,
    deg: i32,
    time: f32,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            r#type: \"width\",
            width: 100,
            height: 100,
            top: 0,
            left: 0,
            deg: 0,
            time: 0.3,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(ty, n) => {
                match ty {
                    \"width\" => self.width = self.width + n,
                    \"height\" => self.height = self.height + n,
                    \"top\" => self.top = self.top + n,
                    \"left\" => self.left = self.left + n,
                    \"rotate\" => self.deg = self.deg + n,
                    _ => (),
                };
                self.time = match ty {
                    \"opacity\" => 3.0,
                    _ => 0.3,
                };
                self.r#type = ty;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        let n = 50;
        html! {
            <>
                <Button dense=true label=\"宽度加\" onclick=&self.link.callback(move |_| Msg::Set(\"width\", n)) />
                <Button dense=true label=\"宽度减\" onclick=&self.link.callback(move |_| Msg::Set(\"width\", -n)) />
                <Button dense=true label=\"高度加\" onclick=&self.link.callback(move |_| Msg::Set(\"height\", n)) />
                <Button dense=true label=\"高度减\" onclick=&self.link.callback(move |_| Msg::Set(\"height\", -n)) />
                <Button dense=true label=\"距上加\" onclick=&self.link.callback(move |_| Msg::Set(\"top\", n)) />
                <Button dense=true label=\"距上减\" onclick=&self.link.callback(move |_| Msg::Set(\"top\", -n)) />
                <Button dense=true label=\"距左加\" onclick=&self.link.callback(move |_| Msg::Set(\"left\", n)) />
                <Button dense=true label=\"距左减\" onclick=&self.link.callback(move |_| Msg::Set(\"left\", -n)) />
                <Button dense=true label=\"顺时针转\" onclick=&self.link.callback(move |_| Msg::Set(\"rotate\", n)) />
                <Button dense=true label=\"逆时针转\" onclick=&self.link.callback(move |_| Msg::Set(\"rotate\", -n)) />
                <Button dense=true label=\"透明度渐变\" onclick=&self.link.callback(move |_| Msg::Set(\"opacity\", 0)) />
                <Flex padding=\"15px 0\">
                    <Animate
                        r#type=self.r#type
                        width=format!(\"{}px\", self.width)
                        height=format!(\"{}px\", self.height)
                        position=\"relative\"
                        top=format!(\"{}px\", self.top)
                        left=format!(\"{}px\", self.left)
                        deg=self.deg
                        time=self.time
                        every_time=true
                        index=9
                    >
                        <Flex width=\"100%\" height=\"100%\" border=true justify=\"center\" align=\"center\">{\"动画对象\"}</Flex>
                    </Animate>
                </Flex>
            </>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("r#type", "动效类型", "String", "", "")}
                        {table_th("width", "宽度，需设置r#type:width", "String", "", "")}
                        {table_th("height", "高度，需设置r#type:height", "String", "", "")}
                        {table_th("margin", "外边距，需设置r#type:margin", "String", "", "")}
                        {table_th("padding", "内边距，需设置r#type:padding", "String", "", "")}
                        {table_th("position", "定位属性，取值同css position", "String", "", "")}
                        {table_th("top", "距离顶部距离，需设置r#type:top", "String", "", "")}
                        {table_th("bottom", "距离底部距离，需设置r#type:bottom", "String", "", "")}
                        {table_th("left", "距离左侧距离，需设置r#type:left", "String", "", "")}
                        {table_th("right", "距离右侧距离，需设置r#type:right", "String", "", "")}
                        {table_th("index", "组件层级，配套position使用", "u32", "0", "")}
                        {table_th("opacity", "透明度渐变，需设置r#type:opacity", "String", "", "")}
                        {table_th("deg", "旋转角度，需设置r#type:rotate", "i32", "0", "")}
                        {table_th("time", "动效持续时间秒", "f32", "0.3", "")}
                        {table_th("scrollbar_x", "是否启用横向滚动条，需设置width才能生效", "bool", "false", "")}
                        {table_th("scrollbar_y", "是否启用竖向滚动条，需设置height才能生效", "bool", "false", "")}
                        {table_th("every_time", "每次组件渲染时属性无变化动效也执行，目前配套r#type:opacity使用", "bool", "false", "")}
                        {table_th("identity", "动效标识，设置新的值，相应动效则执行一次", "String", "", "")}
                    </Flex>
                </div>
                {switch_bottom("theme", "Theme 主题", "appbar", "Appbar 应用栏")}
            </Animate>
        }
    }
}
