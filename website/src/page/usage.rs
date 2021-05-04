use yew::prelude::*;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

pub struct PgUsage {}

impl Component for PgUsage {
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
                    <div class=style("title")>{"create-yew-material-app"}</div>
                    <Text>
                        <a target="_blank" href="https://github.com/alibaba-inc/create-yew-material-app">{"Create Yew Material App"}</a>
                        {" 是一个快速学习yew的集成环境，也是目前用yew创建带有material ui单页web应用的最佳方式"}
                    </Text>
                    <div class=style("subtitle")>{"1、创建新项目"}</div>
                    <ul class=style("describe")>
                        <li>{"注意：下方的npx命令不是拼写错误，它是npm 5.2+ 附带的package运行工具"}</li>
                        {code("
npx create-yew-material-app my-app
cd my-app
npm run start
                        ")}
                    </ul>
                    <div class=style("subtitle")>{"2、项目构建"}</div>
                    <ul class=style("describe")>
                        <li>{"在项目目录下，运行以下命名进行项目构建"}</li>
                        {code("
npm run build
                        ")}
                        <li>{"构建完成后请前往build目录下查看"}</li>
                    </ul>
                    <div class=style("subtitle")>{"3、注意事项"}</div>
                    <p>{"因目前各方面都尚处于早期阶段，因此你需要注意以下几点："}</p>
                    <ul class=style("describe")>
                        <li>{"项目所在rust环境目前只支持nightly版本"}</li>
                        <li>{"yew相关库和yew-material相关库暂时只支持git方式获取，且只能是master分支"}</li>
                        {code("
[dependencies]
yew = { git = \"https://github.com/yewstack/yew/\", branch = \"master\" }
yew-material = { git = \"https://github.com/alibaba-inc/yew-material/\", branch = \"master\" }
                        ")}
                        <li>{"线上环境推荐服务端启用Brotli压缩算法，它将极大的减小wasm文件的网络传输体积"}</li>
                    </ul>
                    {switch_bottom("install", "安装", "style", "Style 样式")}
                </div>
            </Animate>
        }
    }
}
