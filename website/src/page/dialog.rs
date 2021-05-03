use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Open(usize),
    Closed(String),
}

pub struct PgDialog {
    link: ComponentLink<Self>,
    open: Vec<bool>,
    action: String,
}

impl Component for PgDialog {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            open: vec![false, false, false],
            action: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open(idx) => {
                self.open[idx] = true;
                true
            }
            Msg::Closed(action) => {
                self.open = vec![false, false, false];
                self.action = action;
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
                <div class=style("title")>{"Dialog 对话框"}</div>
                <div>{"模态对话框"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    {static_des(8)}
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"dialog\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::dialog::Dialog;
                ")}
                {static_des(2)}
                {notice_des("此组件为响应式组件，组件宽度会适时跟随浏览器文档显示宽度变化而变化")}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Button class="item" label="默认形式" onclick=&self.link.callback(|_| Msg::Open(0)) />
                    <Button class="item" label="拥有标题和按钮" onclick=&self.link.callback(|_| Msg::Open(1)) />
                    <Button class="item" label=format!("关闭事件传递action：{}", &self.action) onclick=&self.link.callback(|_| Msg::Open(2)) />
                    <Dialog
                        open=&self.open[0]
                        closed=&self.link.callback(|action| Msg::Closed(action))
                    >
                        {"默认形式"}
                    </Dialog>
                    <Dialog
                        heading="标题"
                        stacked=true
                        hide_actions=false
                        open=&self.open[1]
                        closed=&self.link.callback(|action| Msg::Closed(action))
                    >
                        <span>{"Love is more than a word，it says so much."}</span>
                        <Button slot="primaryAction" text_button=true label="This button is long"/>
                        <Button slot="secondaryAction" label="这个按钮比上面那个还要长，还要长"/>
                    </Dialog>
                    <Dialog
                        heading="标题"
                        mask_close=false
                        hide_actions=false
                        open=&self.open[2]
                        closed=&self.link.callback(|action| Msg::Closed(action))
                    >
                        <Flex scrollbar_y=true max_height="120px">
                            {"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                        </Flex>
                        <Button
                            slot="secondaryAction"
                            dialog_action="clicked close"
                            text_button=true
                            icon="close"
                            color="#ff4242"
                            label="关闭"
                        />
                        <Button
                            slot="primaryAction"
                            dialog_action="clicked ok"
                            text_button=true
                            icon="check"
                            label="确认"
                        />
                    </Dialog>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Open(usize),
    Closed(String),
}

pub struct Page {
    link: ComponentLink<Self>,
    open: Vec<bool>,
    action: String,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            open: vec![false, false, false],
            action: \"\".into(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open(idx) => {
                self.open[idx] = true;
                true
            }
            Msg::Closed(action) => {
                self.open = vec![false, false, false];
                self.action = action;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Button label=\"默认形式\" onclick=&self.link.callback(|_| Msg::Open(0)) />
                <Button label=\"拥有标题和按钮\" onclick=&self.link.callback(|_| Msg::Open(1)) />
                <Button label=format!(\"关闭事件传递action：{}\", &self.action) onclick=&self.link.callback(|_| Msg::Open(2)) />
                <Dialog
                    open=&self.open[0]
                    closed=&self.link.callback(|action| Msg::Closed(action))
                >
                    {\"默认形式\"}
                </Dialog>
                <Dialog
                    heading=\"标题\"
                    stacked=true
                    hide_actions=false
                    open=&self.open[1]
                    closed=&self.link.callback(|action| Msg::Closed(action))
                >
                    <span>{\"Love is more than a word，it says so much.\"}</span>
                    <Button slot=\"primaryAction\" text_button=true label=\"This button is long\"/>
                    <Button slot=\"secondaryAction\" label=\"这个按钮比上面那个还要长，还要长\"/>
                </Dialog>
                <Dialog
                    heading=\"标题\"
                    mask_close=false
                    hide_actions=false
                    open=&self.open[2]
                    closed=&self.link.callback(|action| Msg::Closed(action))
                >
                    <Flex scrollbar_y=true max_height=\"120px\">
                        {\"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue.\"}
                    </Flex>
                    <Button
                        slot=\"secondaryAction\"
                        dialog_action=\"clicked close\"
                        text_button=true
                        icon=\"close\"
                        color=\"#ff4242\"
                        label=\"关闭\"
                    />
                    <Button
                        slot=\"primaryAction\"
                        dialog_action=\"clicked ok\"
                        text_button=true
                        icon=\"check\"
                        label=\"确认\"
                    />
                </Dialog>
            </>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(2)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("open", "是否打开模态框", "bool", "false", "")}
                        {table_th("heading", "模态框标题", "String", "", "")}
                        {table_th("hide_actions", "隐藏操作栏", "bool", "true", "")}
                        {table_th("stacked", "操作栏组件分层显示", "bool", "false", "")}
                        {table_th("mask_close", "点击蒙层是否允许关闭", "bool", "true", "")}
                        {table_th("closed", "关闭后触发的回调函数", "Callback", "", "")}
                    </Flex>
                    {static_des(4)}
                    {static_des(7)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {other_th("属性", "说明")}
                        {other_th("primaryAction", "主要的操作选项")}
                        {other_th("secondaryAction", "次要的操作选项")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("heading_color", "标题字体颜色")}
                        {other_th("content_color", "内容字体颜色")}
                        {other_th("background", "模态框整体背景颜色")}
                    </Flex>
                </div>
                {switch_bottom("checkbox", "Checkbox 复选框", "divider", "Divider 分割线")}
            </Animate>
        }
    }
}
