use yew::prelude::*;
use yew_material::prelude::*;
use yew_material::select::SelectedRes;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Selected(SelectedRes),
}

pub struct PgSelect {
    link: ComponentLink<Self>,
    result: SelectedRes,
}

impl Component for PgSelect {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            result: (0, "".into()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(res) => {
                self.result = res;
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
                <div class=style("title")>{"Select 选择器"}</div>
                <div>{"下拉列表选择器"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"多重校验：组件拥有多种校验功能"}</li>
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"select\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::select::Select;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="15px 0">
                    <Select class="item">
                        <ListItem>{"请选择"}</ListItem>
                        <ListItem>{"Item 1"}</ListItem>
                        <ListItem>{"Item 2"}</ListItem>
                    </Select>
                    <Select
                        class="item"
                        icon="list"
                        width="145px"
                        outlined=true
                        auto_list_width=true
                    >
                        <ListItem>{"请选择"}</ListItem>
                        <ListItem>{"This is the item is very long"}</ListItem>
                        <ListItem>{"这个子选项内容很长"}</ListItem>
                    </Select>
                    <Select class="item" icon="list" disabled=true>
                        <ListItem>{"Item 1"}</ListItem>
                    </Select>
                    <div>
                        <Select
                            class="item"
                            icon="list"
                            required=true
                            helper="请选择一个选项"
                            validate_message="注意：必选项"
                            value="a"
                        >
                            <ListItem>{"请选择"}</ListItem>
                            <ListItem value="a">{"Item 1"}</ListItem>
                            <ListItem value="b">{"Item 2"}</ListItem>
                        </Select>
                        <Select
                            class="item"
                            fixed_list=true
                            helper=format!("index：{} value：{}", self.result.0, self.result.1)
                            onselected=&self.link.callback(|res| Msg::Selected(res))
                        >
                            <ListItem>{"Fixed ListItem"}</ListItem>
                            <ListItem value="item 1">{"Item 1"}</ListItem>
                            <ListItem value="item 2">{"Item 2"}</ListItem>
                        </Select>
                    </div>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
use yew_material::select::SelectedRes;

pub enum Msg {
    Selected(SelectedRes),
}

pub struct Page {
    link: ComponentLink<Self>,
    result: SelectedRes,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            result: (0, \"\".into()),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(res) => {
                self.result = res;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Select>
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem>{\"Item 1\"}</ListItem>
                    <ListItem>{\"Item 2\"}</ListItem>
                </Select>
                <Select
                    icon=\"list\"
                    width=\"145px\"
                    outlined=true
                    auto_list_width=true
                >
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem>{\"This is the item is very long\"}</ListItem>
                    <ListItem>{\"这个子选项内容很长\"}</ListItem>
                </Select>
                <Select icon=\"list\" disabled=true>
                    <ListItem>{\"Item 1\"}</ListItem>
                </Select>
                <Select
                    icon=\"list\"
                    required=true
                    helper=\"请选择一个选项\"
                    validate_message=\"注意：必选项\"
                    value=\"a\"
                >
                    <ListItem>{\"请选择\"}</ListItem>
                    <ListItem value=\"a\">{\"Item 1\"}</ListItem>
                    <ListItem value=\"b\">{\"Item 2\"}</ListItem>
                </Select>
                <Select
                    fixed_list=true
                    helper=format!(\"index：{} value：{}\", self.result.0, self.result.1)
                    onselected=&self.link.callback(|res| Msg::Selected(res))
                >
                    <ListItem>{\"Fixed ListItem\"}</ListItem>
                    <ListItem value=\"item 1\">{\"Item 1\"}</ListItem>
                    <ListItem value=\"item 2\">{\"Item 2\"}</ListItem>
                </Select>
            </>
        }
    }
}
                    ")}
                    <div class=style("subtitle")>{"Select API"}</div>
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("value", "组件值", "String", "", "")}
                        {table_th("width", "组件宽度", "String", "", "")}
                        {table_th("auto_list_width", "下拉列表宽度是否自适应", "bool", "false", "")}
                        {table_th("fixed_list", "下拉列表位置固定，一般配套Dialog组件使用", "bool", "false", "")}
                        {table_th("icon", "组件图标", "String", "", "")}
                        {table_th("outlined", "带边框模式", "bool", "false", "")}
                        {table_th("helper", "帮助信息", "String", "", "")}
                        {table_th("required", "必填", "bool", "false", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("validate_message", "校验未通过时的提示信息", "String", "", "")}
                        {table_th("validate_trans", "校验前的回调函数，详见From组件", "ValidityRes", "", "")}
                        {table_th("validate_init_render", "组件初始化完成后立即校验", "bool", "false", "")}
                        {table_th("onselected", "列表子项选中后的回调函数", "Callback", "", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    <div class=style("subtitle")>{"ListItem API"}</div>
                    <div>{"详见List组件下ListItem API"}</div>
                    {static_des(5)}
                    <div>{"此组件主题样式继承Textfield组件"}</div>
                </div>
                {switch_bottom("radio", "Radio 单选框", "skeleton", "Skeleton 骨架屏")}
            </Animate>
        }
    }
}
