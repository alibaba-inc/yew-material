use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgIconButton {}

impl Component for PgIconButton {
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
                <div class=style("title")>{"IconButton 图标按钮"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"图标切换：拥有图标切换功能"}</li>
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"icon_button\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::icon_button::IconButton;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="25px">
                    <IconButton icon="sentiment_very_dissatisfied" />
                    <IconButton
                        size="32px"
                        color="#4475ff"
                        on_icon="sentiment_dissatisfied"
                        off_icon="sentiment_very_satisfied"
                        margin="0 25px"
                    />
                    <IconButton margin="0 25px 0 0">
                        <Flex height="100%" justify="center" align="center">
                            <Text size="42px">{"+"}</Text>
                        </Flex>
                    </IconButton>
                    <IconButton size="1.8rem" icon="sentiment_very_dissatisfied" disabled=true />
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<IconButton icon=\"sentiment_very_dissatisfied\" />
<IconButton
    size=\"32px\"
    color=\"#4475ff\"
    on_icon=\"sentiment_dissatisfied\"
    off_icon=\"sentiment_very_satisfied\"
    margin=\"0 25px\"
/>
<IconButton margin=\"0 25px 0 0\">
    <Flex height=\"100%\" justify=\"center\" align=\"center\">
        <Text size=\"42px\">{\"+\"}</Text>
    </Flex>
</IconButton>
<IconButton size=\"1.8rem\" icon=\"sentiment_very_dissatisfied\" disabled=true />
                ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Text auto_wrap=true color="#ff2a2a">
                        {"注意：相应图标标识需自行前往谷歌 "}<a target="_blank" href="https://www.google.com/design/icons">{"material图标库"}</a>{" 查看"}
                    </Text>
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("icon", "标识", "String", "", "")}
                        {table_th("color", "颜色", "String", "", "")}
                        {table_th("size", "尺寸", "String", "", "")}
                        {table_th("margin", "外边距", "String", "", "")}
                        {table_th("on", "是否处于on状态，true：使用on_icon，false：使用off_icon", "bool", "false", "")}
                        {table_th("on_icon", "on状态图标标识", "String", "", "")}
                        {table_th("off_icon", "off状态图标标识", "String", "", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    {static_des(5)}
                    <div>{"此组件主题样式继承Icon组件"}</div>
                </div>
                {switch_bottom("icon", "Icon 图标", "iframe", "Iframe 内嵌子框架")}
            </Animate>
        }
    }
}
