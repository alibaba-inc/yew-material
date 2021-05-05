use yew::prelude::*;
use yew_material::prelude::*;
use yew_material_utils::get_inner_height;
use yew_material_utils::prelude::*;

use crate::styles::code;

pub struct PgIndex {}

impl Component for PgIndex {
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
        let color = "#2196f3";
        let style = Style::easy(json!({
            "content": {
                "color": color,
                "& .logo": {
                    "width": "80px",
                    "height": "80px",
                    "margin": "40px 0 30px 0",
                    "background-image": "url(https://material.io/static/angular/material_logo.1d5336632349ee8653bf.svg), linear-gradient(#fff, #bcb4ff)",
                    "border-radius": "50%",
                    "background-size": "cover",
                    "box-shadow": "0 0 1px 1px #068aff",
                },
                "& .title": {
                    "font-size": "3rem",
                    "font-family": "Roboto",
                    "font-weight": 300,
                    "letter-spacing": ".7rem",
                },
                "& .btn": {
                    "margin": "40px 0",
                    "text-decoration": "none",
                },
                "& .code div": {
                    "width": "500px",
                },
            },
            "@media (max-width: 540px)": {
                "content": {
                    "& span.title": {
                        "font-size": "2rem",
                        "letter-spacing": ".3rem",
                    },
                    "& div.code div": {
                        "width": "100%",
                    },
                },
            },
        }));
        html! {
            <Animate class=style.item("content") r#type="opacity" time=1.2>
                <Flex margin="0 0 -100px 0" align="center" direction="column" height=format!("{}px", get_inner_height())>
                    <div class="logo"/>
                    <Text class="title">{"YEW-MATERIAL"}</Text>
                    <Text size="1.1rem">{"Yew-Material ui框架能让你更便捷的使用Rust语言来开发带有Material Design设计风格的Web应用"}</Text>
                    <a class="btn" href="/zh/install"><Button border_width="1px" border_color=color color=color ripple=color label="开始体验" /></a>
                    <Flex align="center">
                        <Icon icon="code" color=color margin="0 5px 0 0" />
                        <Text>{"通过 Create Yew Material App 快速开始"}</Text>
                    </Flex>
                    <div class="code">
                    {code("
npx create-yew-material-app my-app
cd my-app
npm run start
                    ")}
                    </div>
                </Flex>
            </Animate>
        }
    }
}
