use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Toggle(usize),
}

pub struct PgFlex {
    link: ComponentLink<Self>,
    toggle: Vec<bool>,
}

impl Component for PgFlex {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            toggle: vec![true, true],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle(idx) => {
                self.toggle[idx] = !self.toggle[idx];
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
                <div class=style("title")>{"Flex 布局"}</div>
                <div>{"可以快速实现各种简易flex布局"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"自带切换：可对布局进行隐藏状态切换控制"}</li>
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"flex\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::flex::Flex;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex direction="column" padding="15px 0">
                        <Flex height="80px" margin="0 0 10px 0">
                            <Flex width="100px" border=true>{"width:100px"}</Flex>
                            <Flex
                                grow=1
                                justify="center"
                                border_top=true
                                border_bottom=true
                            >
                                {"grow:1"}
                            </Flex>
                            <Flex align="flex-end" border=true>{"flex-end"}</Flex>
                        </Flex>
                        <Flex direction="column">
                            <Button
                                dense=true
                                label="toggle垂直切换"
                                onclick=&self.link.callback(|_| Msg::Toggle(0))
                            />
                            <Flex
                                toggle=&self.toggle[0]
                                animate=true
                                height="100px"
                                border=true
                                margin="10px 0"
                            >
                                {"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                            </Flex>
                        </Flex>
                        <Flex direction="column">
                            <Button
                                dense=true
                                label="toggle水平切换"
                                onclick=&self.link.callback(|_| Msg::Toggle(1))
                            />
                            <Flex
                                toggle=&self.toggle[1]
                                animate=true
                                animate_type="horizontal"
                                width="250px"
                                border=true
                                margin="10px 0 0 0"
                            >
                                <Flex scrollbar_y=true max_height="80px">
                                    {"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                                </Flex>
                            </Flex>
                        </Flex>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Toggle(usize),
}

pub struct Page {
    link: ComponentLink<Self>,
    toggle: Vec<bool>,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            toggle: vec![true, true],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle(idx) => {
                self.toggle[idx] = !self.toggle[idx];
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <Flex direction=\"column\">
                <Flex height=\"80px\" margin=\"0 0 10px 0\">
                    <Flex width=\"100px\" border=true>{\"width:100px\"}</Flex>
                    <Flex
                        grow=1
                        justify=\"center\"
                        border_top=true
                        border_bottom=true
                    >
                        {\"grow:1\"}
                    </Flex>
                    <Flex align=\"flex-end\" border=true>{\"flex-end\"}</Flex>
                </Flex>
                <Flex direction=\"column\">
                    <Button
                        dense=true
                        label=\"toggle垂直切换\"
                        onclick=&self.link.callback(|_| Msg::Toggle(0))
                    />
                    <Flex
                        toggle=&self.toggle[0]
                        animate=true
                        height=\"100px\"
                        border=true
                        margin=\"10px 0\"
                    >
                        {\"Love is more than a word...\"}
                    </Flex>
                </Flex>
                <Flex direction=\"column\">
                    <Button
                        dense=true
                        label=\"toggle水平切换\"
                        onclick=&self.link.callback(|_| Msg::Toggle(1))
                    />
                    <Flex
                        toggle=&self.toggle[1]
                        animate=true
                        animate_type=\"horizontal\"
                        width=\"250px\"
                        border=true
                        margin=\"10px 0 0 0\"
                    >
                        <Flex scrollbar_y=true max_height=\"80px\">
                            {\"Love is more than a word...\"}
                        </Flex>
                    </Flex>
                </Flex>
            </Flex>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("width", "宽度", "String", "", "")}
                        {table_th("height", "高度", "String", "", "")}
                        {table_th("max_width", "最大宽度", "String", "", "")}
                        {table_th("max_height", "最大高度", "String", "", "")}
                        {table_th("min_width", "最小宽度", "String", "", "")}
                        {table_th("min_height", "最小高度", "String", "", "")}
                        {table_th("toggle", "隐藏状态切换，true为显示，false为隐藏", "bool", "true", "")}
                        {table_th("animate", "是否启用隐藏状态过渡动画，需设置width或height才能生效", "bool", "false", "")}
                        {table_th("animate_type", "隐藏状态切换方向，取值：vertical 垂直，horizontal 水平", "String", "vertical", "")}
                        {table_th("hide", "是否强制隐藏，对应属性：display:none", "bool", "false", "")}
                        {table_th("margin", "外边距", "String", "", "")}
                        {table_th("padding", "内边距", "String", "", "")}
                        {table_th("grow", "剩余空间分配值，取值同css flex-grow", "u32", "0", "")}
                        {table_th("direction", "子元素布局方向，取值同css flex-direction", "String", "row", "")}
                        {table_th("justify", "子元素水平对齐方式，取值同css justify-content", "String", "flex-start", "")}
                        {table_th("align", "子元素垂直对齐方式，取值同css align-items", "String", "stretch", "")}
                        {table_th("border", "是否启用全边框", "bool", "false", "")}
                        {table_th("border_top", "是否启用上边框", "bool", "false", "")}
                        {table_th("border_bottom", "是否启用下边框", "bool", "false", "")}
                        {table_th("border_left", "是否启用左边框", "bool", "false", "")}
                        {table_th("border_right", "是否启用右边框", "bool", "false", "")}
                        {table_th("scrollbar_x", "是否启用横向滚动条，需设置width或max_width才能生效", "bool", "false", "")}
                        {table_th("scrollbar_y", "是否启用竖向滚动条，需设置height或max_height才能生效", "bool", "false", "")}
                        {table_th_auto_theme()}
                    </Flex>
                </div>
                {switch_bottom("divider", "Divider 分割线", "form", "Form 表单")}
            </Animate>
        }
    }
}
