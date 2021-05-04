use crate::route::*;
use crate::styles::*;

use yew::prelude::*;
use yew::utils::window;
use yew_material::{prelude::*, VERSION};
use yew_material_utils::theme::{change_theme, get_theme_ident};
use yew_material_utils::{add_listener, BoolFeatures};

pub enum Msg {
    ChangeTheme,
    ListClick(usize),
    CloseLeftMenu,
}

pub struct Index {
    link: ComponentLink<Self>,
    theme: String,
    list_hide: Vec<bool>,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pathname = window().location().pathname().unwrap();
        let left_menu = match pathname.as_str() {
            "/" => true,
            _ => bool::get_storage("left_menu_hide"),
        };
        Self {
            link,
            theme: get_theme_ident(false),
            list_hide: vec![
                /*0*/ left_menu, //left menu
                /*1*/ false, //easy start
                /*2*/ false, //style and theme
                /*3*/ false, //components
            ],
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::ListClick(index) => {
                self.list_hide[index] = !self.list_hide[index];
                if index == 0 {
                    bool::set_storage("left_menu_hide", self.list_hide[0]);
                }
                true
            }
            Msg::CloseLeftMenu => {
                match self.list_hide[0] {
                    false => self.link.send_message(Msg::ListClick(0)),
                    _ => (),
                }
                false
            }
            Msg::ChangeTheme => {
                self.theme = match self.theme.as_str() {
                    "light" => "dark".into(),
                    _ => "light".into(),
                };
                change_theme(self.theme.clone());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first: bool) {
        if first {
            let link = self.link.to_owned();
            add_listener(
                "_index",
                "close_left_menu",
                Box::new(move || {
                    &link.send_message(Msg::CloseLeftMenu);
                }),
            );
        }
    }

    fn view(&self) -> Html {
        let list_width = "260px";
        let item_height = |n: u32| -> String { format!("{}px", n * 56) };
        html! {
            <Flex id="body" class=style("body") scrollbar_y=true direction="column">
                <Appbar fixed=true>
                    <IconButton icon="menu" color="#fff" slot="navigationIcon" onclick=self.link.callback(|_| Msg::ListClick(0)) />
                    <span slot="title">{"Yew Material-UI"}</span>
                    <span slot="title" class=style("version")>{"v"}{VERSION}</span>
                    <a slot="actionItems" target="_blank" href="https://github.com/alibaba-inc/yew-material">
                        <IconButton color="#fff">
                            <svg viewBox="0 0 36 36">
                                <path d="M18,1.4C9,1.4,1.7,8.7,1.7,17.7c0,7.2,4.7,13.3,11.1,15.5
                                    c0.8,0.1,1.1-0.4,1.1-0.8c0-0.4,0-1.4,0-2.8c-4.5,1-5.5-2.2-5.5-2.2c-0.7-1.9-1.8-2.4-1.8-2.4c-1.5-1,0.1-1,0.1-1
                                    c1.6,0.1,2.5,1.7,2.5,1.7c1.5,2.5,3.8,1.8,4.7,1.4c0.1-1.1,0.6-1.8,1-2.2c-3.6-0.4-7.4-1.8-7.4-8.1c0-1.8,0.6-3.2,1.7-4.4
                                    c-0.2-0.4-0.7-2.1,0.2-4.3c0,0,1.4-0.4,4.5,1.7c1.3-0.4,2.7-0.5,4.1-0.5c1.4,0,2.8,0.2,4.1,0.5c3.1-2.1,4.5-1.7,4.5-1.7
                                    c0.9,2.2,0.3,3.9,0.2,4.3c1,1.1,1.7,2.6,1.7,4.4c0,6.3-3.8,7.6-7.4,8c0.6,0.5,1.1,1.5,1.1,3c0,2.2,0,3.9,0,4.5
                                    c0,0.4,0.3,0.9,1.1,0.8c6.5-2.2,11.1-8.3,11.1-15.5C34.3,8.7,27,1.4,18,1.4z" 
                                />
                            </svg>
                        </IconButton>
                    </a>
                    <IconButton
                        slot="actionItems"
                        color="#fff"
                        on=if self.theme == "light" {true} else {false}
                        on_icon="brightness_7"
                        off_icon="brightness_4"
                        margin="0 0 0 10px"
                        onclick=self.link.callback(|_| Msg::ChangeTheme)
                    />
                </Appbar>
                <Flex>
                    <Flex class="env_mobile_hide" width=list_width animate=true animate_type="horizontal" toggle=!self.list_hide[0] />
                    <Animate
                        class="left_meun"
                        r#type="left"
                        scrollbar_y=true
                        position="fixed"
                        index=3
                        width=list_width
                        bottom="0px"
                        left=if self.list_hide[0] {format!("-{}", list_width)} else {"0px".into()}
                    >
                        <List selected_style=true>
                            <ListItemTemp
                                text="快速上手"
                                icon="local_offer"
                                icon_color="#6e6cf0"
                                arrow=true
                                arrow_more=!self.list_hide[1]
                                onclick=self.link.callback(|_| Msg::ListClick(1))
                            />
                            <Flex animate=true toggle=!self.list_hide[1] height=item_height(2) direction="column">
                                {router_anchor_alias(Routes::Install, "安装")}
                                {router_anchor_alias(Routes::Usage, "使用")}
                            </Flex>
                            <ListItemTemp
                                text="样式与主题"
                                icon="color_lens"
                                icon_color="#ff00c8"
                                arrow=true
                                arrow_more=!self.list_hide[2]
                                onclick=self.link.callback(|_| Msg::ListClick(2))
                            />
                            <Flex animate=true toggle=!self.list_hide[2] height=item_height(2) direction="column">
                                {router_anchor_alias(Routes::Style, "Style")}
                                {router_anchor_alias(Routes::Theme, "Theme")}
                            </Flex>
                            <ListItemTemp
                                text="Components"
                                icon="account_tree"
                                icon_color="#79a410"
                                arrow=true
                                arrow_more=!self.list_hide[3]
                                onclick=self.link.callback(|_| Msg::ListClick(3))
                            />
                            <Flex animate=true toggle=!self.list_hide[3] height=item_height(27) direction="column">
                                {router_anchor(Routes::Animate)}
                                {router_anchor(Routes::Appbar)}
                                {router_anchor(Routes::Button)}
                                {router_anchor(Routes::Checkbox)}
                                {router_anchor(Routes::Dialog)}
                                {router_anchor(Routes::Divider)}
                                {router_anchor(Routes::Flex)}
                                {router_anchor(Routes::Form)}
                                {router_anchor(Routes::FormField)}
                                {router_anchor(Routes::Icon)}
                                {router_anchor(Routes::IconButton)}
                                {router_anchor(Routes::Iframe)}
                                {router_anchor(Routes::Img)}
                                {router_anchor(Routes::InnerHtml)}
                                {router_anchor(Routes::List)}
                                {router_anchor(Routes::Menu)}
                                {router_anchor(Routes::Progress)}
                                {router_anchor(Routes::Radio)}
                                {router_anchor(Routes::Select)}
                                {router_anchor(Routes::Skeleton)}
                                {router_anchor(Routes::Slider)}
                                {router_anchor(Routes::Snackbar)}
                                {router_anchor(Routes::Switch)}
                                {router_anchor(Routes::Tab)}
                                {router_anchor(Routes::Text)}
                                {router_anchor(Routes::Textarea)}
                                {router_anchor(Routes::Textfield)}
                            </Flex>
                            <Flex height="120px" />
                        </List>
                    </Animate>
                    <Flex class=style("container") grow=1 direction="column">
                        {routes_render()}
                    </Flex>
                </Flex>
                <Animate class="footer_box" r#type="margin" index=1 margin=if self.list_hide[0] {"0".into()} else {format!("0 0 0 {}", list_width)}>
                    <Flex class=style("footer") direction="column">
                        <Flex class="footer_items env_mobile_hide" auto_theme=true border_bottom=true padding="30px 0 40px 0">
                            <Flex grow=1 direction="column" max_width="330px">
                                <p>{"相关资源"}</p>
                                <div><a href="https://www.rust-lang.org">{"Rust官网"}</a></div>
                                <div><a href="https://github.com/rustwasm/wasm-bindgen">{"Rust wasm-bindgen Github仓库"}</a></div>
                                <div><a href="https://docs.rs/web-sys">{"Rust web-sys 文档"}</a></div>
                                <div><a href="https://yew.rs">{"Yew官网"}</a></div>
                                <div><a href="https://github.com/material-components/material-components-web-components">{"Material Web-Components Github仓库"}</a></div>
                            </Flex>
                            <Flex grow=1 direction="column" max_width="180px">
                                <p>{"社区"}</p>
                                <div><a href="https://users.rust-lang.org">{"Rust官方论坛"}</a></div>
                                <div><a href="https://discord.gg/rust-lang">{"Rust官方discord"}</a></div>
                                <div><a href="https://rustcc.cn">{"Rust中文社区"}</a></div>
                                <div><a href="https://discord.gg/4mR6magGCs">{"Yew官方discord"}</a></div>
                            </Flex>
                            <Flex grow=1 direction="column">
                                <p>{"帮助"}</p>
                                <Text size="14px">{"Discord：madman.wang#9085"}</Text>
                                <Text size="14px" padding="5px 0 0 0">{"Email：admin@yew-material.cn"}</Text>
                            </Flex>
                        </Flex>
                        <Flex
                            class=style("link")
                            justify="center"
                            padding="20px 0 0 0"
                            direction="column"
                        >
                            <a href="https://madman.wang" target="_blank">
                                <Text vertical_align="bottom">{"作者blog: 王疯子"}</Text>
                                <Icon color="inherit" icon="insert_link" />
                            </a>
                            <Text size="12px" padding="5px 0 0 0">{"备案/许可证号：浙ICP备2021003637号-2"}</Text>
                        </Flex>
                    </Flex>
                </Animate>
            </Flex>
        }
    }
}
