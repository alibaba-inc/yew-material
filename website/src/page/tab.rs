use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Activated(u32),
}

pub struct PgTab {
    link: ComponentLink<Self>,
    index: u32,
}

impl Component for PgTab {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, index: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Activated(idx) => {
                self.index = idx;
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
                <div class=style("title")>{"Tab 选项卡"}</div>
                <div>{"选项卡切换组件"}</div>
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
    \"tab\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::tab::{Tab, TabItem};
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="15px 0">
                    <Tab>
                        <TabItem label="one" />
                        <TabItem label="two" />
                        <TabItem label="three" />
                    </Tab>
                    <Divider />
                    <Tab active_index=1>
                        <TabItem label="壹" icon="accessibility" min_width=true />
                        <TabItem label="贰" icon="exit_to_app" min_width=true />
                        <TabItem label="叁" icon="camera" min_width=true disabled=true />
                    </Tab>
                    <Divider />
                    <Tab active_index=1>
                        <TabItem icon="sentiment_very_satisfied" indicator_icon="donut_large" is_fading_indicator=true />
                        <TabItem icon="exit_to_app" indicator_icon="donut_large" is_fading_indicator=true />
                        <TabItem icon="camera" indicator_icon="donut_large" is_fading_indicator=true />
                    </Tab>
                    <Divider />
                    <Tab>
                        <TabItem has_image_icon=true>
                            <Img slot="icon" src="/static/website/favicon.ico" />
                        </TabItem>
                        <TabItem has_image_icon=true>
                            <Img slot="icon" src="/static/website/favicon.ico" />
                        </TabItem>
                        <TabItem has_image_icon=true>
                            <Img slot="icon" src="/static/website/favicon.ico" />
                        </TabItem>
                    </Tab>
                    <Divider />
                    <Flex border=true direction="column" padding="15px" width="300px">
                        <Tab onactivated=&self.link.callback(|res| Msg::Activated(res))>
                            <TabItem label="item-1" icon="camera" is_min_width_indicator=true stacked=true />
                            <TabItem label="item-2" icon="camera" is_min_width_indicator=true stacked=true />
                            <TabItem label="item-3" icon="camera" is_min_width_indicator=true stacked=true />
                            <TabItem label="item-4" icon="camera" is_min_width_indicator=true stacked=true />
                            <TabItem label="item-5" icon="camera" is_min_width_indicator=true stacked=true />
                            <TabItem label="item-6" icon="camera" is_min_width_indicator=true stacked=true />
                        </Tab>
                        <Divider />
                        <div>{"onactivated index："}{self.index}</div>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
pub enum Msg {
    Activated(u32),
}

pub struct Page {
    link: ComponentLink<Self>,
    index: u32,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, index: 0 }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Activated(idx) => {
                self.index = idx;
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Tab>
                    <TabItem label=\"one\" />
                    <TabItem label=\"two\" />
                    <TabItem label=\"three\" />
                </Tab>
                <Divider />
                <Tab active_index=1>
                    <TabItem label=\"壹\" icon=\"accessibility\" min_width=true />
                    <TabItem label=\"贰\" icon=\"exit_to_app\" min_width=true />
                    <TabItem label=\"叁\" icon=\"camera\" min_width=true disabled=true />
                </Tab>
                <Divider />
                <Tab active_index=1>
                    <TabItem icon=\"sentiment_very_satisfied\" indicator_icon=\"donut_large\" is_fading_indicator=true />
                    <TabItem icon=\"exit_to_app\" indicator_icon=\"donut_large\" is_fading_indicator=true />
                    <TabItem icon=\"camera\" indicator_icon=\"donut_large\" is_fading_indicator=true />
                </Tab>
                <Divider />
                <Tab>
                    <TabItem has_image_icon=true>
                        <Img slot=\"icon\" src=\".../favicon.ico\" />
                    </TabItem>
                    <TabItem has_image_icon=true>
                        <Img slot=\"icon\" src=\".../favicon.ico\" />
                    </TabItem>
                    <TabItem has_image_icon=true>
                        <Img slot=\"icon\" src=\".../favicon.ico\" />
                    </TabItem>
                </Tab>
                <Divider />
                <Flex border=true direction=\"column\" padding=\"15px\" width=\"300px\">
                    <Tab onactivated=&self.link.callback(|res| Msg::Activated(res))>
                        <TabItem label=\"item-1\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                        <TabItem label=\"item-2\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                        <TabItem label=\"item-3\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                        <TabItem label=\"item-4\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                        <TabItem label=\"item-5\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                        <TabItem label=\"item-6\" icon=\"camera\" is_min_width_indicator=true stacked=true />
                    </Tab>
                    <Divider />
                    <div>{\"onactivated index：\"}{self.index}</div>
                </Flex>
            </>
        }
    }
}
                    ")}
                    <div class=style("subtitle")>{"Tab API"}</div>
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("width", "组件宽度", "String", "", "")}
                        {table_th("active_index", "初始激活的子选项索引", "u32", "0", "")}
                        {table_th("onactivated", "子项选激活后的回调函数", "Callback", "", "")}
                    </Flex>
                    <div class=style("subtitle")>{"TabItem API"}</div>
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("label", "文本信息", "String", "", "")}
                        {table_th("icon", "图标", "String", "", "")}
                        {table_th("has_image_icon", "是否启用图片位，占位符：icon", "bool", "false", "")}
                        {table_th("indicator_icon", "指示器图标", "String", "", "")}
                        {table_th("is_fading_indicator", "指示器效果为淡入淡出，默认为滑动效果", "bool", "false", "")}
                        {table_th("min_width", "子选项卡宽度尽可能的缩小", "bool", "false", "")}
                        {table_th("is_min_width_indicator", "收缩指示器的宽度为内容的宽度", "bool", "false", "")}
                        {table_th("stacked", "图标是否位于文本上方", "bool", "false", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                    </Flex>
                    <div class=style("subtitle")>{"TabItem Slots"}</div>
                    {static_des(7)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {other_th("属性", "说明")}
                        {other_th("icon", "图标位")}
                    </Flex>
                    <div class=style("subtitle")>{"TabItem Theme"}</div>
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("color", "文本颜色")}
                        {other_th("icon_color", "图标颜色")}
                        {other_th("activated", "激活状态下整体颜色")}
                        {other_th("ripple", "涟漪动效颜色")}
                    </Flex>
                </div>
                {switch_bottom("switch", "Switch 开关", "text", "Text 文本")}
            </Animate>
        }
    }
}
