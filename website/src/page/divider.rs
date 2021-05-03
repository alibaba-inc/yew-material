use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgDivider {}

impl Component for PgDivider {
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
                <div class=style("title")>{"Divider 分割线"}</div>
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
    \"divider\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::divider::Divider;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex direction="column" padding="15px 50px">
                        <span>
                            {"第一句"}<Divider vertical=true />{"Love is more than a word，it says so much."}
                        </span>
                        <Divider color="#68df00" />
                        <span>
                            {"第二句"}<Divider vertical=true />{"When I see these four letters, I almost feel your touch."}
                        </span>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<span>
    {\"第一句\"}<Divider vertical=true />{\"Love is more than a word，it says so much.\"}
</span>
<Divider color=\"#68df00\" />
<span>
    {\"第二句\"}<Divider vertical=true />{\"When I see these four letters, I almost feel your touch.\"}
</span>
                ")}
                    {static_des(3)}
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("color", "分割线颜色", "String", "", "")}
                        {table_th("margin", "分割线外间距", "String", "", "")}
                        {table_th("vertical", "是否是垂直分割线", "bool", "", "")}
                        {table_th("role", "配套ListItem组件使用，取值：list_item", "String", "", "")}
                        {table_th("padded", "是否带有两边内边距，需配套role:list_item使用", "bool", "false", "")}
                        {table_th("inset", "是否带有更大的左内边距，详见List组件，需配套role:list_item使用", "bool", "false", "")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "分割线颜色")}
                    </Flex>
                </div>
                {switch_bottom("dialog", "Dialog 对话框", "flex", "Flex 布局")}
            </Animate>
        }
    }
}
