use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Change(usize, bool),
}

pub struct PgSwitch {
    link: ComponentLink<Self>,
    status: Vec<bool>,
}

impl Component for PgSwitch {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            status: vec![false, true],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(idx, status) => {
                self.status[idx] = status;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Switch 开关"}</div>
                <div>{"用于某个功能在开与关两种状态间切换"}</div>
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
    \"switch\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::switch::Switch;
                ")}
                {static_des(2)}
                <Flex padding="26px 15px" border_top=true border_bottom=true>
                    <Switch
                        checked=self.status[0]
                        onchange=&self.link.callback(|(status, _)| Msg::Change(0, status))
                    />
                    <Flex padding="0 34px">
                        <Switch
                            checked=self.status[1]
                            onchange=&self.link.callback(|(status, _)| Msg::Change(1, status))
                        />
                    </Flex>
                    <Switch disabled=true />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Switch />
<Switch checked=true />
<Switch disabled=true />
                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("on_value", "开启状态下组件值", "String", "on", "")}
                        {table_th("off_value", "关闭状态下组件值", "String", "off", "")}
                        {table_th("checked", "是否选中，即开启状态", "bool", "false", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("onchange", "值变更后回调函数", "Callback", "", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("checked", "选中状态下的整体颜色")}
                        {other_th("unchecked", "未选中状态下的整体颜色")}
                        {other_th("disabled", "禁用状态下的整体颜色")}
                    </Flex>
                </div>
            {switch_bottom("snackbar", "Snackbar 提醒框", "tab", "Tab 选项卡")}
            </Animate>
        }
    }
}
