use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgIcon {}

impl Component for PgIcon {
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
                <div class=style("title")>{"Icon 图标"}</div>
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
    \"icon\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::icon::Icon;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="15px">
                    <Icon icon="sentiment_very_dissatisfied" margin="0 25px 0 0" />
                    <Icon size="64px" color="red" icon="exit_to_app" />
                    <Icon size="2.2rem" color="#4475ff" icon="accessibility" margin="0 25px" />
                    <Icon size="3em" color="rgba(176, 158, 0, .75)" icon="camera" />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Icon icon=\"sentiment_very_dissatisfied\" margin=\"0 25px 0 0\" />
<Icon size=\"64px\" color=\"red\" icon=\"exit_to_app\" />
<Icon size=\"2.2rem\" color=\"#4475ff\" icon=\"accessibility\" margin=\"0 25px\" />
<Icon size=\"3em\" color=\"rgba(176, 158, 0, .75)\" icon=\"camera\" />
                ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Text auto_wrap=true color="#ff2a2a">
                        {"注意：相应图标标识需自行前往谷歌 "}<a target="_blank" href="https://www.google.com/design/icons">{"material图标库"}</a>{" 查看"}
                    </Text>
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("icon", "标识", "String", "", "")}
                        {table_th("color", "颜色", "String", "", "")}
                        {table_th("size", "尺寸", "String", "", "")}
                        {table_th("margin", "外边距", "String", "", "")}
                        {table_th("vertical_align", "垂直对齐方式，取值同css vertical-align", "", "", "")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "颜色")}
                    </Flex>
                </div>
                {switch_bottom("formfield", "FormField 表单域", "iconbutton", "IconButton 图标按钮")}
            </Animate>
        }
    }
}
