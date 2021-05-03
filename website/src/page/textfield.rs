use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgTextfield {}

impl Component for PgTextfield {
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
                <div class=style("title")>{"Textfield 输入框"}</div>
                <div>{"用户文本输入框"}</div>
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
    \"textfield\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::textfield::Textfield;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Textfield class="item" focus=true placeholder="请输入" />
                    <Textfield class="item" label="提示信息" placeholder="请输入" />
                    <Textfield
                        class="item"
                        r#type="email"
                        label="提示信息"
                        prefix="邮箱："
                        suffix="eg:abc@123.com"
                        icon="email"
                        auto_validate=true
                    />
                    <Flex padding="10px 0">
                        <Textfield
                            width="100%"
                            outlined=true
                            label="提示信息"
                            helper="帮助信息展示在这里"
                            helper_persistent=true
                            max_length=60
                            char_counter=true
                            icon_trailing="content_paste"
                        />
                    </Flex>
                    <Textfield
                        class="item"
                        label="请输入数字"
                        pattern="[2-8]+"
                        required=true
                        auto_validate=true
                        validate_init_render=true
                        validate_message="只能输入2-8的数字组合哦~"
                    />
                    <Textfield
                        class="item"
                        width="230px"
                        r#type="number"
                        label="请输入数字"
                        min="999"
                        max="99999"
                        auto_validate=true
                        validate_message="只能输入999-99999的数字区间哦~"
                    />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Textfield focus=true placeholder=\"请输入\" />
<Textfield label=\"提示信息\" placeholder=\"请输入\" />
<Textfield
    r#type=\"email\"
    label=\"提示信息\"
    prefix=\"邮箱：\"
    suffix=\"eg:abc@123.com\"
    icon=\"email\"
    auto_validate=true
/>
<Flex padding=\"10px 0\">
    <Textfield
        width=\"100%\"
        outlined=true
        label=\"提示信息\"
        helper=\"帮助信息展示在这里\"
        helper_persistent=true
        max_length=60
        char_counter=true
        icon_trailing=\"content_paste\"
    />
</Flex>
<Textfield
    label=\"请输入数字\"
    pattern=\"[2-8]+\"
    required=true
    auto_validate=true
    validate_init_render=true
    validate_message=\"只能输入2-8的数字组合哦~\"
/>
<Textfield
    width=\"230px\"
    r#type=\"number\"
    label=\"请输入数字\"
    min=\"999\"
    max=\"99999\"
    auto_validate=true
    validate_message=\"只能输入999-99999的数字区间哦~\"
/>
                    ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("name", "同input name属性，一般配套表单取值使用", "String", "", "")}
                        {table_th("value", "组件值", "String", "", "")}
                        {table_th("r#type", "组件类型，取值：text、number、search、tel、url、email、password", "String", "text", "")}
                        {table_th("width", "宽度", "String", "", "")}
                        {table_th("focus", "是否获得焦点", "bool", "false", "")}
                        {table_th("label", "标签提示信息", "String", "", "")}
                        {table_th("placeholder", "普通提示信息", "String", "", "")}
                        {table_th("prefix", "前缀信息", "String", "", "")}
                        {table_th("suffix", "尾缀信息", "String", "", "")}
                        {table_th("icon", "前缀图标", "String", "", "")}
                        {table_th("icon_trailing", "尾缀图标", "String", "", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("char_counter", "是否显示计数器，需设置max_length属性", "bool", "false", "")}
                        {table_th("outlined", "带边框模式", "bool", "false", "")}
                        {table_th("helper", "帮助信息，组件拥有焦点时显示", "String", "", "")}
                        {table_th("helper_persistent", "帮助信息始终显示", "bool", "false", "")}
                        {table_th("required", "必填", "bool", "false", "")}
                        {table_th("max_length", "组件值最大长度", "u32", "", "")}
                        {table_th("pattern", "正则表达式", "String", "", "")}
                        {table_th("min", "最小值，一般配套r#type=number使用", "String", "", "")}
                        {table_th("max", "最大值，一般配套r#type=number使用", "String", "", "")}
                        {table_th("validate_message", "校验未通过时的提示信息", "String", "", "")}
                        {table_th("validate_trans", "校验前的回调函数，详见From组件", "ValidityRes", "", "")}
                        {table_th("validate_init_render", "组件初始化完成后立即校验", "bool", "false", "")}
                        {table_th("auto_validate", "值改变后自动校验", "bool", "false", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "字体颜色")}
                        {other_th("background", "整体背景色")}
                        {other_th("error_color", "校验未通过时相关交互颜色")}
                        {other_th("label_color", "标签提示信息颜色")}
                        {other_th("label_focus_color", "焦点状态下标签提示信息颜色")}
                        {other_th("icon_color", "图标颜色")}
                        {other_th("disabled_color", "禁用状态下字体颜色")}
                        {other_th("disabled_background", "禁用状态下整体背景色")}
                        {other_th("radius", "边框圆角，默认4px")}
                        {other_th("border_color", "outlined模式下边框颜色")}
                        {other_th("border_hover_color", "outlined模式下鼠标悬浮后边框颜色")}
                        {other_th("border_disabled_color", "outlined模式下禁用状态边框颜色")}
                    </Flex>
                </div>
            {switch_bottom("textarea", "Textarea 文本域", "install", "安装")}
            </Animate>
        }
    }
}
