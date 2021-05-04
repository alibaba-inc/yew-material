use crate::styles::*;
use yew::prelude::*;
use yew_material::prelude::*;

pub struct PgInstall {}

impl Component for PgInstall {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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
                <div class=style("exhibition")>
                    <div class=style("title")>{"基础设施安装"}</div>
                    <Text>
                        {"因yew框架是由rust语言开发而来，而yew-material ui框架则又是在 "}
                        <a target="_blank" href="https://github.com/material-components/material-components-web-components">{"Material Web Components"}</a>
                        {" 基础上开发的，所以在开始使用本ui框架之前，你需要先确认你的开发环境已安装好以下设施："}
                    </Text>
                    <div class=style("subtitle")>{"1、Rust环境"}</div>
                    <ul class=style("describe")>
                        <li>{"前往 "}<a target="_blank" href="https://www.rust-lang.org">{"rust官网"}</a>{"，根据相应教程安装好rust环境"}</li>
                        <li>{"将rust切换至nightly版本，stable版暂不支持"}</li>
                        <li>{"国内用户因网络问题，配置文件可使用以下内容"}</li>
                        {code("
[source.crates-io]
replace-with = \"tuna\"
[source.tuna]
registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"
[http]
check-revoke = false
                        ")}
                    </ul>
                    <div class=style("subtitle")>{"2、Node环境"}</div>
                    <ul class=style("describe")>
                        <li>{"前往 "}<a target="_blank" href="https://nodejs.org">{"node官网"}</a>{"，根据相应教程安装好node环境"}</li>
                        <li>{"以防止意外发生，请确保node和npm是最新版本"}</li>
                        <li>{"国内用户因网络问题，可将npm切换至淘宝镜像"}</li>
                        {code("
npm config set registry https://registry.npm.taobao.org
                        ")}
                    </ul>
                    <Flex border_top=true border_bottom=true margin="65px 0 80px 0" padding="15px 0" justify="center">
                        <Text size="18px"><a href="/zh/usage">{"环境已准备好，进行下一步"}</a></Text>
                    </Flex>
                </div>
            </Animate>
        }
    }
}
