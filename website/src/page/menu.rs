use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Selected(u32),
}

pub struct PgMenu {
    link: ComponentLink<Self>,
    result: u32,
}

impl Component for PgMenu {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, result: 2 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(idx) => {
                self.result = idx;
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
                <div class=style("title")>{"Menu 下拉菜单"}</div>
                <div>{"下拉菜单选择器"}</div>
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
    \"menu\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::menu::Menu;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="15px 0">
                    <Menu
                        class="item"
                        corner="TOP_END"
                        anchor=html!{
                            <Button label="menu" />
                        }
                    >
                        <ListItem>{"请选择"}</ListItem>
                        <ListItem>{"Item 1"}</ListItem>
                        <ListItem>{"Item 2"}</ListItem>
                    </Menu>
                    <Menu
                        class="item"
                        full_width=true
                        anchor=html!{
                            <Button label="This is a long menu" />
                        }
                    >
                        <ListItem>{"请选择"}</ListItem>
                        <ListItem>{"Item 1"}</ListItem>
                        <ListItem>{"Item 2"}</ListItem>
                    </Menu>
                    <Menu
                        class="item"
                        anchor=html!{<Button label=format!("selected：{}", self.result) />}
                        onselected=&self.link.callback(|res| Msg::Selected(res))
                    >
                        <ListItem>{"请选择"}</ListItem>
                        <ListItem>{"Item 1"}</ListItem>
                        <ListItem selected=true>{"Item 2"}</ListItem>
                    </Menu>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Selected(u32),
}

pub struct Page {
    link: ComponentLink<Self>,
    result: u32,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, result: 2 }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(idx) => {
                self.result = idx;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Menu
                    corner=\"TOP_END\"
                    anchor=html!{
                        <Button label=\"menu\" />
                    }
                >
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem>{\"Item 1\"}</ListItem>
                    <ListItem>{\"Item 2\"}</ListItem>
                </Menu>
                <Menu
                    class=\"item\"
                    full_width=true
                    anchor=html!{
                        <Button label=\"This is a long menu\" />
                    }
                >
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem>{\"Item 1\"}</ListItem>
                    <ListItem>{\"Item 2\"}</ListItem>
                </Menu>
                <Menu
                    anchor=html!{<Button label=format!(\"selected：{}\", self.result) />}
                    onselected=&self.link.callback(|res| Msg::Selected(res))
                >
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem>{\"Item 1\"}</ListItem>
                    <ListItem selected=true>{\"Item 2\"}</ListItem>
                </Menu>
            </>
        }
    }
}
                    ")}
                    <div class=style("subtitle")>{"Menu API"}</div>
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("anchor", "触发下拉菜单的锚点组件", "Html", "", "")}
                        {table_th("full_width", "下拉菜单宽度铺满锚点组件宽度", "bool", "false", "")}
                        {table_th("corner", "下拉菜单位置设置，取值：TOP_LEFT、TOP_RIGHT、BOTTOM_LEFT、BOTTOM_RIGHT、TOP_START、TOP_END、BOTTOM_START、BOTTOM_END", "String", "BOTTOM_LEFT", "")}
                        {table_th("fixed", "下拉菜单位置固定，一般配套Dialog组件使用", "bool", "false", "")}
                        {table_th("onselected", "列表子项选中后的回调函数", "Callback", "", "")}
                    </Flex>
                    <div class=style("subtitle")>{"ListItem API"}</div>
                    <div>{"详见List组件下ListItem API"}</div>
                </div>
                {switch_bottom("list", "List 列表", "progress", "Progress 进度条")}
            </Animate>
        }
    }
}
