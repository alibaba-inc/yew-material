use crate::styles::style as self_style;
use yew::prelude::*;
use yew::utils::document;
use yew_material::prelude::*;
use yew_material::styles::style;
use yew_material_utils::theme;

pub enum Msg {
    Set(usize),
    Reload,
}

pub struct ExpAppbar {
    link: ComponentLink<Self>,
    status: Vec<bool>,
}

fn set_scrollbar() {
    let body = document().body().unwrap();
    body.set_class_name(&style("scrollbar"));
}

impl Component for ExpAppbar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        set_scrollbar();
        Self {
            link,
            status: vec![false, false, false, false],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(idx) => {
                self.status[idx] = !self.status[idx];
                true
            }
            Msg::Reload => {
                set_scrollbar();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, first: bool) {
        if first {
            let link = self.link.to_owned();
            theme::on_change(
                &"_theme_on_change".to_string(),
                Box::new(move || link.send_message(Msg::Reload)),
            );
        }
    }

    fn view(&self) -> Html {
        let content = || {
            html! {
                <Flex direction="column">
                    <Flex class=self_style("exhibition")>
                        <Button class="item" onclick=&self.link.callback(|_| Msg::Set(0))>
                            {"toggle fixed："}{self.status[0]}
                        </Button>
                        <Button class="item" onclick=&self.link.callback(|_| Msg::Set(1))>
                            {"toggle dense："}{self.status[1]}
                        </Button>
                        <Button class="item" onclick=&self.link.callback(|_| Msg::Set(2))>
                            {"toggle center_title："}{self.status[2]}
                        </Button>
                        <Button class="item" onclick=&self.link.callback(|_| Msg::Set(3))>
                            {"toggle prominent："}{self.status[3]}
                        </Button>
                    </Flex>
                    <Text size="20px" indent="50px" auto_wrap=true padding="0 20px">
                        {"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                    </Text>
                    <Img width="250px" margin="15px 0" src="https://ss0.bdstatic.com/70cFuHSh_Q1YnxGkpoWK1HF6hhy/it/u=2498791300,2171078002&fm=26&gp=0.jpg" />
                </Flex>
            }
        };
        html! {
            <Animate r#type="opacity" time=1.2>
                <Appbar
                    fixed=&self.status[0]
                    dense=&self.status[1]
                    center_title=&self.status[2]
                    prominent=&self.status[3]
                >
                    <IconButton color="#fff" icon="menu" slot="navigationIcon" />
                    <div slot="title">{"Title"}</div>
                    <IconButton color="#fff" icon="file_download" slot="actionItems" />
                    <IconButton color="#fff" icon="print" slot="actionItems" />
                    {
                        if !self.status[0] {
                            content()
                        } else {
                            html!{<></>}
                        }
                    }
                </Appbar>
                {
                    if self.status[0] {
                        content()
                    } else {
                        html!{<></>}
                    }
                }
            </Animate>
        }
    }
}
