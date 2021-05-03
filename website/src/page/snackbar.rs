use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Open(usize),
}

pub struct PgSnackbar {
    link: ComponentLink<Self>,
    open: Vec<bool>,
}

impl Component for PgSnackbar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            open: vec![false, false, false],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open(idx) => {
                self.open = vec![false, false, false];
                self.open[idx] = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.open = vec![false, false, false];
        true
    }

    fn view(&self) -> Html {
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Snackbar 提醒框"}</div>
                <div>{"提醒框组件可展示成功、警告、错误等反馈信息"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    {static_des(8)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"snackbar\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::snackbar::Snackbar;
                ")}
                {static_des(2)}
                {notice_des("此组件为响应式组件，当浏览器文档显示宽度大于600px时，组件渲染位置为顶部，反之在底部")}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Button class="item" label="默认形式" onclick=&self.link.callback(|_| Msg::Open(0)) />
                    <Button class="item" label="居左带图标" onclick=&self.link.callback(|_| Msg::Open(1)) />
                    <Button class="item" label="居右双层形式" onclick=&self.link.callback(|_| Msg::Open(2)) />
                    <Snackbar timeout=-1 label="提醒信息：默认形式" open=&self.open[0] />
                    <Snackbar label="提醒信息：居左带图标" open=&self.open[1] align="left">
                        <Icon slot="action" icon="error" color="#5da913" />
                    </Snackbar>
                    <Snackbar label="提醒信息：居右双层形式" open=&self.open[2] align="right" stacked=true>
                        <Button slot="action" dense=true icon="remove_red_eye" label="前往查看" />
                    </Snackbar>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Open(usize),
}

pub struct Page {
    link: ComponentLink<Self>,
    open: Vec<bool>,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            open: vec![false, false, false],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open(idx) => {
                self.open = vec![false, false, false];
                self.open[idx] = true;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Button label=\"默认形式\" onclick=&self.link.callback(|_| Msg::Open(0)) />
                <Button label=\"居左带图标\" onclick=&self.link.callback(|_| Msg::Open(1)) />
                <Button label=\"居右双层形式\" onclick=&self.link.callback(|_| Msg::Open(2)) />
                <Snackbar label=\"提醒信息：默认形式\" open=&self.open[0] />
                <Snackbar label=\"提醒信息：居左带图标\" open=&self.open[1] align=\"left\">
                    <Icon slot=\"action\" icon=\"error\" color=\"#5da913\" />
                </Snackbar>
                <Snackbar label=\"提醒信息：居右双层形式\" open=&self.open[2] align=\"right\" stacked=true>
                    <Button slot=\"action\" dense=true icon=\"remove_red_eye\" label=\"前往查看\" />
                </Snackbar>
            </>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(2)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("open", "是否展示提醒框", "bool", "false", "")}
                        {table_th("label", "提醒框文本内容", "String", "", "")}
                        {table_th("align", "提醒框水平位置，取值：left，center，right", "String", "center", "")}
                        {table_th("stacked", "内容与操作部分是否分层显示", "bool", "false", "")}
                        {table_th("timeout", "自动关闭时长毫秒，取值：4000~10000，设为-1则禁止自动关闭", "i32", "5000", "")}
                        {table_th("closed", "关闭后触发的回调函数", "Callback", "", "")}
                    </Flex>
                    {static_des(4)}
                    {static_des(7)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {other_th("属性", "说明")}
                        {other_th("action", "额外操作选项位")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "文本颜色")}
                        {other_th("background", "背景颜色")}
                    </Flex>
                </div>
                {switch_bottom("slider", "Slider 滑动条", "switch", "Switch 开关")}
            </Animate>
        }
    }
}
