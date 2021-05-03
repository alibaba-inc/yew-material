use yew::prelude::*;
use yew_material::animate::Animate;

pub enum Msg {
    Render(String),
}

pub struct PgInstall {
    link: ComponentLink<Self>,
}

impl Component for PgInstall {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Render(text) => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            //self.link.send_message(Msg::Render);
        }
    }

    fn view(&self) -> Html {
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class="box bs chat-box" >
                    { "你好-YEW: "}
                </div>
            </Animate>
        }
    }
}
