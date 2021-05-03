use crate::page::*;
use crate::styles::*;
use web_sys::window;
use yew::prelude::*;
use yew_material::prelude::*;
use yew_material_utils::prelude::*;
use yew_material_utils::{dispatch, element_by_id, get_inner_width};
use yew_router::{prelude::*, Switch};
use yewtil::function_component;

#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to = "/zh/usage"]
    Usage,
    #[to = "/zh/install"]
    Install,
    #[to = "/zh/style"]
    Style,
    #[to = "/zh/theme"]
    Theme,
    #[to = "/zh/button"]
    Button,
    #[to = "/zh/textfield"]
    Textfield,
    #[to = "/zh/textarea"]
    Textarea,
    #[to = "/zh/text"]
    Text,
    #[to = "/zh/img"]
    Img,
    #[to = "/zh/divider"]
    Divider,
    #[to = "/zh/skeleton"]
    Skeleton,
    #[to = "/zh/snackbar"]
    Snackbar,
    #[to = "/zh/dialog"]
    Dialog,
    #[to = "/zh/flex"]
    Flex,
    #[to = "/zh/animate"]
    Animate,
    #[to = "/zh/appbar"]
    Appbar,
    #[to = "/zh/radio"]
    Radio,
    #[to = "/zh/checkbox"]
    Checkbox,
    #[to = "/zh/formfield"]
    FormField,
    #[to = "/zh/form"]
    Form,
    #[to = "/zh/switch"]
    Switch,
    #[to = "/zh/list"]
    List,
    #[to = "/zh/select"]
    Select,
    #[to = "/zh/menu"]
    Menu,
    #[to = "/zh/slider"]
    Slider,
    #[to = "/zh/tab"]
    Tab,
    #[to = "/zh/icon_button"]
    IconButton,
    #[to = "/zh/icon"]
    Icon,
    #[to = "/zh/progress"]
    Progress,
    #[to = "/zh/iframe"]
    Iframe,
    #[to = "/zh/innerhtml"]
    InnerHtml,
}

pub fn switch_routes(switch: Routes, alias: &str) -> (Html, Html) {
    let (name, page) = match switch {
        Routes::Install => ("Install", html! {<PgInstall />}),
        Routes::Usage => ("Usage", html! {<PgUsage />}),
        Routes::Button => ("Button", html! {<PgButton />}),
        Routes::Text => ("Text", html! {<PgText />}),
        Routes::Img => ("Img", html! {<PgImg />}),
        Routes::Divider => ("Divider", html! {<PgDivider />}),
        Routes::Skeleton => ("Skeleton", html! {<PgSkeleton />}),
        Routes::Snackbar => ("Snackbar", html! {<PgSnackbar />}),
        Routes::Dialog => ("Dialog", html! {<PgDialog />}),
        Routes::Flex => ("Flex", html! {<PgFlex />}),
        Routes::Animate => ("Animate", html! { <PgAnimate /> }),
        Routes::Appbar => ("Appbar", html! { <PgAppbar /> }),
        Routes::Radio => ("Radio", html! {<PgRadio />}),
        Routes::Textfield => ("Textfield", html! {<PgTextfield />}),
        Routes::Textarea => ("Textarea", html! {<PgTextarea />}),
        Routes::Select => ("Select", html! {<PgSelect />}),
        Routes::Checkbox => ("Checkbox", html! {<PgCheckbox />}),
        Routes::Form => ("Form", html! {<PgForm />}),
        Routes::Switch => ("Switch", html! {<PgSwitch />}),
        Routes::List => ("List", html! {<PgList />}),
        Routes::Menu => ("Menu", html! {<PgMenu />}),
        Routes::Slider => ("Slider", html! {<PgSlider />}),
        Routes::Tab => ("Tab", html! {<PgTab />}),
        Routes::Icon => ("Icon", html! {<PgIcon />}),
        Routes::IconButton => ("Icon_Button", html! {<PgIconButton />}),
        Routes::FormField => ("FormField", html! {<PgFormField />}),
        Routes::Progress => ("Progress", html! {<PgProgress />}),
        Routes::Iframe => ("Iframe", html! {<PgIframe />}),
        Routes::InnerHtml => ("InnerHtml", html! {<PgInnerHtml />}),
        Routes::Style => ("Style", html! {<PgStyle />}),
        Routes::Theme => ("Theme", html! {<PgTheme />}),
    };
    let anchors = html! {
        <RouterAnchor<Routes> route=switch>
            <ListItemTemp
                text=if alias!="" {alias.into()} else {name.replace("_", "")}
                pathname=name.to_lowercase()
                onclick=Callback::from(|_| {
                    element_by_id("body").set_scroll_top(0);
                    if get_inner_width() < 600.0 {
                        dispatch("close_left_menu");
                    }
                })
            />
        </RouterAnchor<Routes>>
    };
    (page, anchors)
}

pub fn router_anchor(switch: Routes) -> Html {
    let (_, anchors) = switch_routes(switch, "");
    anchors
}

pub fn router_anchor_alias(switch: Routes, alias: &str) -> Html {
    let (_, anchors) = switch_routes(switch, alias);
    anchors
}

pub fn routes_render() -> Html {
    html! {
        <Router<Routes, ()>
            render = Router::render(|switch: Routes| {
                let (page, _) = switch_routes(switch, "");
                page
            })
        />
    }
}

pub fn router_selected(name: &String) -> bool {
    let pathname = window().unwrap().location().pathname().unwrap();
    if pathname == format!("/zh/{}", name) {
        true
    } else {
        false
    }
}

#[function_component(ListItemTemp)]
pub fn list_item_temp(
    #[prop_or_default] onclick: &Callback<MouseEvent>,
    #[prop_or_else(|| "".into())] icon: String,
    #[prop_or_else(|| "".into())] icon_color: String,
    #[prop_or_default] text: String,
    #[prop_or_default] pathname: String,
    #[prop_or_else(|| false)] arrow: bool,
    #[prop_or_default] arrow_more: bool,
) -> Html {
    html! {
        <ListItem
            selected=router_selected(&pathname)
            graphic="icon"
            has_meta=arrow
            selector_ignore=arrow
            onclick=onclick
        >
            {
                if icon != "" {
                    html!{<Icon color=icon_color icon=icon slot="graphic" />}
                } else {
                    html!{<></>}
                }
            }
            <span>{text}</span>
            {
                if arrow {
                    html!{
                        <Animate slot="meta" r#type="rotate" deg=if !arrow_more {180} else {0}>
                            <Icon icon="expand_more" />
                        </Animate>
                    }
                } else {
                    html!{<></>}
                }
            }
        </ListItem>
    }
}

pub fn static_des(i: u32) -> Html {
    let des = match i {
        1 => html! {<div class=style("subtitle")>{"使用方法"}</div>},
        2 => html! {<div class=style("subtitle")>{"组件展示"}</div>},
        3 => html! {<div class=style("subtitle")>{"API"}</div>},
        4 => html! {<div class=style("subtitle")>{"Slots"}</div>},
        5 => html! {<div class=style("subtitle")>{"Theme"}</div>},
        7 => html! {<span>{"提供给children使用的额外组件占位符"}</span>},
        8 => html! {<li>{"响应式：在不同屏幕尺寸下会以最合适的效果展示"}</li>},
        9 => html! {<li>{"主题组件：组件部分样式可跟随主题切换"}</li>},
        _ => html! {<p>{"当前最新版本主要提供了以下功能："}</p>},
    };
    des
}

pub fn link_des(_url: &str) -> Html {
    html! {
        <div class=style("link")>
            <a
                target="_blank"
                href=format!("https://github.com/alibaba-inc/yew-material/blob/master/website/src/page/{}.rs", Url::get_path(1))
            >
                <Text vertical_align="bottom">{"编辑或查看Github本页源码"}</Text>
                <Icon color="inherit" icon="insert_link" />
            </a>
        </div>
    }
}

pub fn api_des(i: u32) -> Html {
    let des = match i {
        4 => "id class slot onclick",
        3 => "id class slot",
        2 => "id class",
        1 => "id",
        _ => "",
    };
    html! {
        <Text>{"支持基础属性："}{des}</Text>
    }
}

pub fn notice_des(des: &str) -> Html {
    html! {
        <div><Text color="red">{"注意："}{des}</Text></div>
    }
}

pub fn table_th(name: &str, des: &str, typ: &str, def: &str, ver: &str) -> Html {
    html! {
        <Flex border_bottom=true>
            <Flex padding="10px 10px 10px 0" min_width="120px">{name}</Flex>
            <Flex padding="10px 10px 10px 0" grow=1>{des}</Flex>
            <Flex padding="10px 10px 10px 0" min_width="65px">{typ}</Flex>
            <Flex padding="10px 10px 10px 0" width="84px">{def}</Flex>
            <Flex padding="10px 0" width="50px">{ver}</Flex>
        </Flex>
    }
}

pub fn table_th_auto_theme() -> Html {
    table_th(
        "auto_theme",
        "自动跟随主题，建议在切换主题后该组件外观未能生效的情况下开启",
        "bool",
        "false",
        "",
    )
}

pub fn other_th(name: &str, des: &str) -> Html {
    html! {
        <Flex border_bottom=true>
            <Flex padding="10px 10px 10px 0" width="160px">{name}</Flex>
            <Flex padding="10px 10px 10px 0" grow=1>{des}</Flex>
        </Flex>
    }
}

pub fn switch_bottom(left_link: &str, left_text: &str, right_link: &str, right_text: &str) -> Html {
    html! {
        <Flex grow=1 margin="60px -8px 0 -8px">
            <Flex class=style("link")>
                <a href=format!("/zh/{}", left_link)>
                    <Icon color="inherit" icon="navigate_before" />
                    <Text vertical_align="bottom">{left_text}</Text>
                </a>
            </Flex>
            <Flex grow=1 />
            <Flex class=style("link")>
                <a href=format!("/zh/{}", right_link)>
                    <Text vertical_align="bottom">{right_text}</Text>
                    <Icon color="inherit" icon="navigate_next" />
                </a>
            </Flex>
        </Flex>
    }
}
