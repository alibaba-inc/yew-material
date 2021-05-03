use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgText {}

impl Component for PgText {
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
                <div class=style("title")>{"Text 文本"}</div>
                <div>{"文本组件拥有在复杂情况下快速展示文本能力"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"自动换行：可强制文本内容自动换行"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"text\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::text::Text;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Flex direction="column">
                        <Text padding="15px 0 0 0">{"实例文本"}</Text>
                        <Text padding="20px 30px" size="0.8rem" color="#5a7d08">
                            <Text size="16px" color="#0e75f3">{"实例文本："}</Text>{"Love is more than a word..."}
                        </Text>
                        <Text indent="30px" auto_wrap=true>
                            {"实例文本，首行文本缩进30px，自动换行：Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                        </Text>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
<Text>{\"实例文本\"}</Text>
<Text padding=\"20px 30px\" size=\"0.8rem\" color=\"#5a7d08\">
    <Text size=\"16px\" color=\"#0e75f3\">{\"实例文本：\"}</Text>{\"Love is more than a word...\"}
</Text>
<Text indent=\"30px\" auto_wrap=true>
    {\"实例文本，首行文本缩进30px，自动换行：Love is more than a word，it says so much...\"}
</Text>
                    ")}
                    {static_des(3)}
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("size", "文本字体大小", "String", "", "")}
                        {table_th("color", "文本字体颜色", "String", "", "")}
                        {table_th("padding", "文本内间距", "String", "", "")}
                        {table_th("align", "文本水平对齐方式，取值同css text-align", "String", "", "")}
                        {table_th("vertical_align", "文本垂直对齐方式，取值同css vertical-align", "", "", "")}
                        {table_th("line_height", "文本行高", "String", "1.6", "")}
                        {table_th("indent", "首行文本缩进距离", "String", "", "")}
                        {table_th("auto_wrap", "是否自动换行，设置为true该组件会变为块级元素", "bool", "false", "")}
                    </Flex>
                </div>
                {switch_bottom("tab", "Tab 选项卡", "textarea", "Textarea 文本域")}
            </Animate>
        }
    }
}
