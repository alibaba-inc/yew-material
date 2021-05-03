use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgButton {}

impl Component for PgButton {
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
                <div class=style("title")>{"Button 按钮"}</div>
                <div>{"按钮组件允许用户通过单击进行操作和选择"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"图标按钮：material图标在按钮内是否前后展示"}</li>
                    <li>{"加载按钮：按钮内在图标位置是否展示加载中动画"}</li>
                    <li>{"主题按钮：可自定义不同主题下按钮外观样式"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"button\",
    //如需使用按钮图标则需要配置此项
    \"icon\",
    //如需开启加载状态则需要配置此项
    \"progress_circular\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::button::Button;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Button class="item" label="默认按钮"/>
                    <Button class="item" icon="code" label="包含图标"/>
                    <Button class="item" icon="code" trailing_icon=true label="图标在右"/>
                    <Button class="item" loading=true label="加载状态"/>
                    <Button class="item" loading=true trailing_icon=true label="加载状态"/>
                    <Button class="item" icon="code" disabled=true label="禁用状态"/>
                    <Button class="item" border_width="3px" border_color="#2c9a1b" color="#fff" background="#036508" ripple="#dbff00">
                        {"自定义"}<Icon icon="stars" color="#6796ff"/>
                    </Button>
                    <Button class="item" text_button=true label="文本按钮" />
                    <Flex class="item" width="100%">
                        <Button icon="code" full_width=true label="宽度铺满"/>
                    </Flex>
                    <Button class="item" icon="code" dense=true label="小尺寸"/>
                    <Button class="item" unelevated=true dense=true label="无阴影"/>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Button label=\"默认按钮\"/>
<Button icon=\"code\" label=\"包含图标\"/>
<Button icon=\"code\" trailing_icon=true label=\"图标在右\"/>
<Button loading=true label=\"加载状态\"/>
<Button loading=true trailing_icon=true label=\"加载状态\"/>
<Button icon=\"code\" disabled=true label=\"禁用状态\"/>
<Button 
    border_width=\"3px\" 
    border_color=\"#2c9a1b\" 
    color=\"#fff\" 
    background=\"#036508\" 
    ripple=\"#dbff00\"
>
    {\"自定义\"}<Icon icon=\"stars\" color=\"#6796ff\"/>
</Button>
<Button text_button=true label=\"文本按钮\" />
<Flex width=\"100%\">
    <Button icon=\"code\" full_width=true label=\"宽度铺满\"/>
</Flex>
<Button icon=\"code\" dense=true label=\"小尺寸\"/>
<Button unelevated=true dense=true label=\"无阴影\"/>
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("label", "按钮显示的文字内容", "String", "", "")}
                        {table_th("color", "按钮显示的字体颜色", "String", "#fff", "")}
                        {table_th("background", "按钮背景色，整体颜色", "String", "#6200ee", "")}
                        {table_th("text_button", "以文本形式显示按钮", "bool", "false", "")}
                        {table_th("icon", "按钮显示的图标", "String", "", "")}
                        {table_th("trailing_icon", "按钮图标置后，及右侧", "bool", "false", "")}
                        {table_th("border_width", "按钮边框宽度", "String", "", "")}
                        {table_th("border_color", "按钮边框颜色", "String", "", "")}
                        {table_th("full_width", "按钮宽度铺满父节点", "bool", "false", "")}
                        {table_th("loading", "按钮加载状态，会覆盖icon所在位置", "bool", "false", "")}
                        {table_th("disabled", "按钮禁用状态", "bool", "false", "")}
                        {table_th("dense", "小尺寸按钮", "bool", "false", "")}
                        {table_th("ripple", "按钮点击后涟漪动效颜色", "String", "#fff", "")}
                        {table_th("unelevated", "取消按钮阴影效果", "bool", "false", "")}
                        {table_th("r#type", "配套Form组件的按钮type属性，取值：submit、button、reset", "String", "button", "")}
                        {table_th("dialog_action", "配套Aialog组件关闭事件时使用，详见Aialog组件", "String", "", "")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "按钮字体颜色")}
                        {other_th("background", "按钮整体颜色")}
                        {other_th("text_button_color", "文本按钮字体颜色")}
                        {other_th("ripple", "按钮点击后涟漪动效颜色")}
                        {other_th("disabled_color", "禁用状态下按钮字体颜色")}
                        {other_th("disabled_background", "禁用状态下按钮整体颜色")}
                    </Flex>
                </div>
            {switch_bottom("appbar", "Appbar 应用栏", "checkbox", "Checkbox 复选框")}
            </Animate>
        }
    }
}
