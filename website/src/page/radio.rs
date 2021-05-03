use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Change(usize, bool),
}

pub struct PgRadio {
    link: ComponentLink<Self>,
    status: Vec<bool>,
}

impl Component for PgRadio {
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
                self.status = vec![false, false];
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
                <div class=style("title")>{"Radio 单选框"}</div>
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
    \"radio\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::radio::Radio;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Radio
                        class="item"
                        checked=self.status[0]
                        onchange=&self.link.callback(|status| Msg::Change(0, status))
                    />
                    <Radio
                        class="item"
                        checked=self.status[1]
                        onchange=&self.link.callback(|status| Msg::Change(1, status))
                    />
                    <Radio class="item" disabled=true />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Radio />
<Radio checked=true />
<Radio disabled=true />
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("value", "组件值", "String", "", "")}
                        {table_th("checked", "是否选中", "bool", "false", "")}
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
            {switch_bottom("progress", "Progress 进度条", "select", "Select 选择器")}
            </Animate>
        }
    }
}
