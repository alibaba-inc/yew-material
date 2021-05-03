use yew::prelude::*;
use yew_material::list::SelectedRes;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub enum Msg {
    Selected(&'static str, SelectedRes),
}

pub struct PgList {
    link: ComponentLink<Self>,
    single: i32,
    multi: Vec<Vec<i32>>,
}

impl Component for PgList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            single: 1,
            multi: vec![vec![], vec![], vec![]],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(idx, (index, added, removed)) => {
                if idx == "multi" {
                    self.multi = vec![index, added, removed];
                } else {
                    self.single = index[0];
                }
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
                <div class=style("title")>{"List 列表"}</div>
                <div>{"通用列表"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"多种交互：拥有单选或多选等多种交互功能"}</li>
                    {static_des(9)}
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"list\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::list::{List, ListItem};
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true padding="15px 0">
                    <Flex direction="column" border=true>
                        <List width="100%" onselected=&self.link.callback(|res| Msg::Selected("single", res))>
                            <ListItem twoline=true graphic="avatar" has_meta=true>
                                {"single 0"}
                                <span slot="secondary">{"secondary line"}</span>
                                <Icon slot="graphic" size="42px" icon="account_circle" />
                                <Icon slot="meta" icon="favorite" />
                            </ListItem>
                            <Divider role="list_item" padded=true />
                            <ListItem r#type="radio" selected=true>{"single 1"}</ListItem>
                            <Divider role="list_item" />
                            <ListItem r#type="radio" left=true>{"single 2"}</ListItem>
                            <Divider role="list_item" padded=true inset=true />
                            <ListItem r#type="radio" left=true disabled=true>{"single 3"}</ListItem>
                            <Divider role="list_item" />
                            <ListItem>{"single 4"}</ListItem>
                        </List>
                        <Flex border_top=true padding="6px 16px 10px 16px">
                            {format!("single selected index：{:?}", self.single)}
                        </Flex>
                    </Flex>
                    <Flex direction="column" border=true margin="10px 0 0 0">
                        <List width="100%" item_height="42px" multi=true activatable=true onselected=&self.link.callback(|res| Msg::Selected("multi", res))>
                            <ListItem>{"multi 0"}</ListItem>
                            <Divider role="list_item" padded=true />
                            <ListItem r#type="check" selected=true>{"multi 1"}</ListItem>
                            <Divider role="list_item" />
                            <ListItem r#type="check" left=true>{"multi 2"}</ListItem>
                            <Divider role="list_item" inset=true />
                            <ListItem r#type="check" left=true disabled=true>{"multi 3"}</ListItem>
                            <Divider role="list_item" />
                            <ListItem r#type="radio" group="a">{"multi 4-1"}</ListItem>
                            <Divider role="list_item" padded=true />
                            <ListItem r#type="radio" group="a">{"multi 4-2"}</ListItem>
                            <Divider role="list_item" />
                            <ListItem r#type="radio" group="b">{"multi 5-1"}</ListItem>
                            <Divider role="list_item" padded=true />
                            <ListItem r#type="radio" group="b">{"multi 5-2"}</ListItem>
                        </List>
                        <Flex border_top=true padding="6px 16px 10px 16px">
                            {format!("multi selected index：{:?} - added：{:?} - removed：{:?}", self.multi[0], self.multi[1], self.multi[2])}
                        </Flex>
                    </Flex>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
use yew_material::list::SelectedRes;

pub enum Msg {
    Selected(&'static str, SelectedRes),
}

pub struct Page {
    link: ComponentLink<Self>,
    single: i32,
    multi: Vec<Vec<i32>>,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            single: 1,
            multi: vec![vec![], vec![], vec![]],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(idx, (index, added, removed)) => {
                if idx == \"multi\" {
                    self.multi = vec![index, added, removed];
                } else {
                    self.single = index[0];
                }
                true
            }
        }
    }
    ...
    fn view(&self) -> Html {
        html! {
            <>
                <Flex direction=\"column\" border=true>
                    <List width=\"100%\" onselected=&self.link.callback(|res| Msg::Selected(\"single\", res))>
                        <ListItem twoline=true graphic=\"avatar\" has_meta=true>
                            {\"single 0\"}
                            <span slot=\"secondary\">{\"secondary line\"}</span>
                            <Icon slot=\"graphic\" size=\"42px\" icon=\"account_circle\" />
                            <Icon slot=\"meta\" icon=\"favorite\" />
                        </ListItem>
                        <Divider role=\"list_item\" padded=true />
                        <ListItem r#type=\"radio\" selected=true>{\"single 1\"}</ListItem>
                        <Divider role=\"list_item\" />
                        <ListItem r#type=\"radio\" left=true>{\"single 2\"}</ListItem>
                        <Divider role=\"list_item\" padded=true inset=true />
                        <ListItem r#type=\"radio\" left=true disabled=true>{\"single 3\"}</ListItem>
                        <Divider role=\"list_item\" />
                        <ListItem>{\"single 4\"}</ListItem>
                    </List>
                    <Flex border_top=true padding=\"6px 16px 10px 16px\">
                        {format!(\"single selected index：{:?}\", self.single)}
                    </Flex>
                </Flex>
                <Flex direction=\"column\" border=true margin=\"10px 0 0 0\">
                    <List width=\"100%\" item_height=\"42px\" multi=true activatable=true onselected=&self.link.callback(|res| Msg::Selected(\"multi\", res))>
                        <ListItem>{\"multi 0\"}</ListItem>
                        <Divider role=\"list_item\" padded=true />
                        <ListItem r#type=\"check\" selected=true>{\"multi 1\"}</ListItem>
                        <Divider role=\"list_item\" />
                        <ListItem r#type=\"check\" left=true>{\"multi 2\"}</ListItem>
                        <Divider role=\"list_item\" inset=true />
                        <ListItem r#type=\"check\" left=true disabled=true>{\"multi 3\"}</ListItem>
                        <Divider role=\"list_item\" />
                        <ListItem r#type=\"radio\" group=\"a\">{\"multi 4-1\"}</ListItem>
                        <Divider role=\"list_item\" padded=true />
                        <ListItem r#type=\"radio\" group=\"a\">{\"multi 4-2\"}</ListItem>
                        <Divider role=\"list_item\" />
                        <ListItem r#type=\"radio\" group=\"b\">{\"multi 5-1\"}</ListItem>
                        <Divider role=\"list_item\" padded=true />
                        <ListItem r#type=\"radio\" group=\"b\">{\"multi 5-2\"}</ListItem>
                    </List>
                    <Flex border_top=true padding=\"6px 16px 10px 16px\">
                        {format!(\"multi selected index：{:?} - added：{:?} - removed：{:?}\", self.multi[0], self.multi[1], self.multi[2])}
                    </Flex>
                </Flex>
            </>
        }
    }
}
                    ")}
                    <div class=style("subtitle")>{"List API"}</div>
                    {api_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("multi", "多选功能", "bool", "false", "")}
                        {table_th("activatable", "子项启用激活状态", "bool", "false", "")}
                        {table_th("noninteractive", "禁用列表所有交互，以便纯数据展示", "bool", "false", "")}
                        {table_th("width", "列表外宽度", "String", "", "")}
                        {table_th("item_height", "子组件ListItem高度", "String", "", "")}
                        {table_th("item_graphic_margin", "占位符slot:graphic的外边距", "String", "", "")}
                        {table_th("onselected", "列表子项选中后的回调函数", "Callback", "", "")}
                        {table_th_auto_theme()}
                    </Flex>
                    <div class=style("subtitle")>{"ListItem API"}</div>
                    {api_des(4)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("r#type", "类型，取值：radio、check", "String", "", "")}
                        {table_th("value", "子项值，仅在Select组件下生效", "String", "", "")}
                        {table_th("group", "子项分组，仅在r#type:radio和Menu组件下生效", "String", "", "")}
                        {table_th("left", "控件是否在左侧显示，仅在r#type:radio和r#type:check下生效", "String", "", "")}
                        {table_th("disabled", "禁用", "bool", "false", "")}
                        {table_th("twoline", "是否带有副标题，占位符：secondary", "bool", "false", "")}
                        {table_th("graphic", "左侧占位符类型，取值：avatar、icon、medium、large、control，占位符：graphic", "String", "", "")}
                        {table_th("multiple_graphics", "允许多个左侧占位符", "bool", "false", "")}
                        {table_th("has_meta", "是否启用右侧占位符，占位符：meta", "bool", "false", "")}
                        {table_th("noninteractive", "禁用所有交互，以便纯数据展示", "bool", "false", "")}
                        {table_th("selected", "是否已被选中", "bool", "false", "")}
                        {table_th("activated", "激活状态", "bool", "false", "")}
                    </Flex>
                    <div class=style("subtitle")>{"ListItem Slots"}</div>
                    {static_des(7)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {other_th("属性", "说明")}
                        {other_th("secondary", "副标题位")}
                        {other_th("graphic", "左图形位")}
                        {other_th("meta", "右图标位")}
                    </Flex>
                    {static_des(5)}
                    <Flex direction="column" border_top=true>
                        {other_th("属性", "说明")}
                        {other_th("background", "整体背景色")}
                        {other_th("ripple", "子项点击后涟漪动效颜色")}
                        {other_th("hover", "子项鼠标悬停后颜色")}
                        {other_th("selected", "子项选中状态下颜色")}
                    </Flex>
                </div>
                {switch_bottom("innerhtml", "InnerHtml", "menu", "Menu 下拉菜单")}
            </Animate>
        }
    }
}
