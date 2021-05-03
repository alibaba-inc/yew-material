use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgAppbar {}

impl Component for PgAppbar {
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
                <div class=style("title")>{"Appbar 应用栏"}</div>
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
    \"appbar\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::appbar::Appbar;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Iframe height="300px" src="/?exp=appbar" />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Set(usize),
    Reload,
}

pub struct Page {
    link: ComponentLink<Self>,
    status: Vec<bool>,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            status: vec![false, false, false, false],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(idx) => {
                self.status[idx] = !self.status[idx];
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        let content = || {
            html! {
                <Flex direction=\"column\">
                    <div>
                        <Button onclick=&self.link.callback(|_| Msg::Set(0))>
                            {\"toggle fixed：\"}{self.status[0]}
                        </Button>
                        <Button onclick=&self.link.callback(|_| Msg::Set(1))>
                            {\"toggle dense：\"}{self.status[1]}
                        </Button>
                        <Button onclick=&self.link.callback(|_| Msg::Set(2))>
                            {\"toggle center_title：\"}{self.status[2]}
                        </Button>
                        <Button onclick=&self.link.callback(|_| Msg::Set(3))>
                            {\"toggle prominent：\"}{self.status[3]}
                        </Button>
                    </div>
                    <Text size=\"20px\" indent=\"50px\" auto_wrap=true padding=\"0 20px\">
                        {\"Love is more than a word...\"}
                    </Text>
                    <Img width=\"250px\" margin=\"15px 0\" src=\"https://ss0.bdstatic.com/...\" />
                </Flex>
            }
        };
        html! {
            <>
                <Appbar
                    fixed=&self.status[0]
                    dense=&self.status[1]
                    center_title=&self.status[2]
                    prominent=&self.status[3]
                >
                    <IconButton icon=\"menu\" slot=\"navigationIcon\" />
                    <div slot=\"title\">{\"Title\"}</div>
                    <IconButton icon=\"file_download\" slot=\"actionItems\" />
                    <IconButton icon=\"print\" slot=\"actionItems\" />
                    {
                        if !self.status[0] {
                            content()
                        } else {
                            html!{<></>}
                        }
                    }
                </Appbar>
                {
                    if self.status[0] {
                        content()
                    } else {
                        html!{<></>}
                    }
                }
            </>
        }
    }
}
                ")}
                    {static_des(3)}
                    {api_des(2)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("fixed", "是否固定在顶部", "bool", "false", "")}
                        {table_th("center_title", "标题是否居中显示", "bool", "false", "")}
                        {table_th("prominent", "标题是否独行显示", "bool", "false", "")}
                        {table_th("dense", "是否启用小尺寸", "bool", "false", "")}
                        {table_th("index", "层级", "u32", "4", "")}
                    </Flex>
                    {static_des(4)}
                    {static_des(7)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {other_th("属性", "说明")}
                        {other_th("navigationIcon", "左侧图标位")}
                        {other_th("title", "标题位")}
                        {other_th("actionItems", "右侧功能扩展位")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("font_family", "字体")}
                        {other_th("background", "整体背景色")}
                        {other_th("shadow", "下边框阴影，只在属性fixed=true时有效")}
                    </Flex>
                </div>
                {switch_bottom("animate", "Animate 动画", "button", "Button 按钮")}
            </Animate>
        }
    }
}
