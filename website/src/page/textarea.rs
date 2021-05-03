use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgTextarea {}

impl Component for PgTextarea {
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
                <div class=style("title")>{"Textarea 文本域"}</div>
                <div>{"用于多行文本输入"}</div>
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
    \"textarea\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::textarea::Textarea;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Textarea class="item" focus=true placeholder="请输入" />
                    <Flex padding="0 0 10px 0">
                        <Textarea
                            class="item"
                            label="请输入"
                            required=true
                            auto_validate=true
                            validate_init_render=true
                            validate_message="这是一个必填项哦~"
                        />
                    </Flex>
                    <Textarea
                        width="100%"
                        rows=4
                        outlined=true
                        label="提示信息"
                        helper="帮助信息展示在这里"
                        helper_persistent=true
                        max_length=60
                        char_counter=true
                    />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Textarea focus=true placeholder=\"请输入\" />
<Textarea
    label=\"请输入\"
    required=true
    auto_validate=true
    validate_init_render=true
    validate_message=\"这是一个必填项哦~\"
/>
<Textarea
    width=\"100%\"
    rows=4
    outlined=true
    label=\"提示信息\"
    helper=\"帮助信息展示在这里\"
    helper_persistent=true
    max_length=60
    char_counter=true
/>

                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("value", "组件值", "String", "", "")}
                        {table_th("rows", "组将行数", "u32", "2", "")}
                        {table_th("width", "宽度", "String", "", "")}
                        {table_th("focus", "是否获得焦点", "bool", "false", "")}
                        {table_th("label", "标签提示信息", "String", "", "")}
                        {table_th("placeholder", "普通提示信息", "String", "", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("char_counter", "是否显示计数器，需设置max_length属性", "bool", "false", "")}
                        {table_th("outlined", "带边框模式", "bool", "false", "")}
                        {table_th("helper", "帮助信息，组件拥有焦点时显示", "String", "", "")}
                        {table_th("helper_persistent", "帮助信息始终显示", "bool", "false", "")}
                        {table_th("required", "必填", "bool", "false", "")}
                        {table_th("max_length", "组件值最大长度", "u32", "", "")}
                        {table_th("validate_message", "校验未通过时的提示信息", "String", "", "")}
                        {table_th("validate_trans", "校验前的回调函数，详见From组件", "ValidityRes", "", "")}
                        {table_th("validate_init_render", "组件初始化完成后立即校验", "bool", "false", "")}
                        {table_th("auto_validate", "值改变后自动校验", "bool", "false", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    {static_des(5)}
                    <div>{"此组件主题样式继承Textfield组件"}</div>
                </div>
            {switch_bottom("text", "Text 文本", "textfield", "Textfield 输入框")}
            </Animate>
        }
    }
}
