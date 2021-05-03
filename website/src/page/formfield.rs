use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgFormField {}

impl Component for PgFormField {
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
                <div class=style("title")>{"FormField 表单域"}</div>
                <div>{"常用于对各种表单组件增加label项"}</div>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"formfield\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::formfield::FormField;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <FormField label="This is a checkbox" margin="5px 0 0 0">
                        <Checkbox />
                    </FormField>
                    <div>
                        <FormField align_end=true label="This is a radio" margin="0 15px">
                            <Radio />
                        </FormField>
                        <FormField align_end=true label="This is a radio">
                            <Radio checked=true />
                        </FormField>
                    </div>
                    <FormField label="This is a switch" cursor="default" padding="10px 0 15px 15px">
                        <Switch />
                    </FormField>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<FormField label=\"This is a checkbox\">
    <Checkbox />
</FormField>
<div>
    <FormField align_end=true label=\"This is a radio\" margin=\"0 15px\">
        <Radio />
    </FormField>
    <FormField align_end=true label=\"This is a radio\">
        <Radio checked=true />
    </FormField>
</div>
<FormField label=\"This is a switch\" cursor=\"default\" padding=\"10px 0 0 15px\">
    <Switch />
</FormField>
                ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("label", "文本", "String", "", "")}
                        {table_th("align_end", "相关组件是否在后，即文本在前", "bool", "false", "")}
                        {table_th("nowrap", "关闭文本自动换行", "bool", "false", "")}
                        {table_th("margin", "外边距", "String", "", "")}
                        {table_th("padding", "内边距", "String", "", "")}
                        {table_th("cursor", "鼠标指针类型，取值同css cursor", "String", "pointer", "")}
                    </Flex>
                </div>
                {switch_bottom("form", "Form 表单", "icon", "Icon 图标")}
            </Animate>
        }
    }
}
