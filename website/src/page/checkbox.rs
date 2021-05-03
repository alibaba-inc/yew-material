use yew::prelude::*;
use yew_material::prelude::*;
use yew_material_utils::{json, Item, Style};

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Change(usize, bool),
    All(bool),
}

pub struct PgCheckbox {
    link: ComponentLink<Self>,
    status: Vec<bool>,
}

impl Component for PgCheckbox {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            status: vec![false, true, false],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(idx, status) => {
                self.status[idx] = status;
                false
            }
            Msg::All(status) => {
                self.status = vec![status, status, status];
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let self_style = Style::easy(json!({
            "sample": {
                "padding": "15px 0",
                "& .field": {
                    "margin-right": "10px"
                },
                "& .item": {
                    "margin-left": "10px"
                },
            }
        }));

        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Checkbox 复选框"}</div>
                <div>{"用于在多个选项中选中某些选项"}</div>
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
    \"checkbox\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::checkbox::Checkbox;
                ")}
                {static_des(2)}
                <Flex class=self_style.item("sample") border_top=true border_bottom=true>
                    <FormField class="field" label="全选">
                        <Checkbox indeterminate=true onchange=&self.link.callback(|status| Msg::All(status)) />
                    </FormField>
                    <Checkbox
                        class="item"
                        checked=self.status[0]
                        onchange=&self.link.callback(|status| Msg::Change(0, status))
                    />
                    <Checkbox
                        class="item"
                        checked=self.status[1]
                        onchange=&self.link.callback(|status| Msg::Change(1, status))
                    />
                    <Checkbox
                        class="item"
                        disabled=true
                        checked=self.status[2]
                        onchange=&self.link.callback(|status| Msg::Change(2, status))
                    />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Change(usize, bool),
    All(bool),
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
            status: vec![false, true, false],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(idx, status) => {
                self.status[idx] = status;
                false
            }
            Msg::All(status) => {
                self.status = vec![status, status, status];
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <FormField label=\"全选\">
                    <Checkbox indeterminate=true onchange=&self.link.callback(|(status, _)| Msg::All(status)) />
                </FormField>
                <Checkbox
                    checked=self.status[0]
                    onchange=&self.link.callback(|(status, _)| Msg::Change(0, status))
                />
                <Checkbox
                    checked=self.status[1]
                    onchange=&self.link.callback(|(status, _)| Msg::Change(1, status))
                />
                <Checkbox
                    disabled=true
                    checked=self.status[2]
                    onchange=&self.link.callback(|(status, _)| Msg::Change(2, status))
                />
            </>
        }
    }
}
                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("value", "组件值", "String", "", "")}
                        {table_th("checked", "是否选中", "bool", "false", "")}
                        {table_th("indeterminate", "不确定状态模式，一般用于复选框全选操作按钮", "bool", "false", "")}
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
            {switch_bottom("button", "Button 按钮", "dialog", "Dialog 对话框")}
            </Animate>
        }
    }
}
