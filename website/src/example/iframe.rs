use yew::prelude::*;
use yew::utils::document;
use yew_material::prelude::*;
use yew_material_utils::{
    json,
    style::{Item, Style},
    theme,
};

pub enum Msg {
    // Open,
    // Closed,
    Change(i32),
    Reload,
}

pub struct ExpIframe {
    link: ComponentLink<Self>,
    // open: bool,
    line: i32,
}

impl Component for ExpIframe {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::easy(json!({
            "body": {
                "overflow": "hidden",
                "min-height": "60px",
            }
        }));
        let body = document().body().unwrap();
        body.set_class_name(&style.item("body"));
        Self {
            link,
            // open: false,
            line: 3,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::Open => {
            //     self.open = true;
            //     true
            // }
            // Msg::Closed => {
            //     self.open = false;
            //     true
            // }
            Msg::Change(i) => {
                let line = self.line + i;
                self.line = if line < 0 { 0 } else { line };
                true
            }
            Msg::Reload => true,
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
        let mut i = 0;
        let mut line = vec![];
        while i < self.line {
            line.push(i);
            i += 1;
        }
        html! {
            <Animate r#type="opacity" time=1.2>
                <Flex border=true padding="24px" direction="column">
                    <Flex direction="column">
                        {
                            for line.iter().map(|_| html!{
                                <Flex border=true margin="5px 0" padding="5px 10px">
                                    {"Love is more than a word，it says so much."}
                                </Flex>
                            })
                        }
                    </Flex>
                    <Flex justify="center" margin="10px 0 0 0">
                        <Button dense=true icon="add" label="加" onclick=&self.link.callback(|_| Msg::Change(1)) />
                        <Divider vertical=true />
                        <Button dense=true icon="delete" label="减" onclick=&self.link.callback(|_| Msg::Change(-1)) />
                        // <Divider vertical=true />
                        // <Button label="Open Dialog" onclick=&self.link.callback(|_| Msg::Open) />
                    </Flex>
                </Flex>
                // <Dialog
                //     heading="标题"
                //     hide_actions=false
                //     open=&self.open
                //     closed=&self.link.callback(|_| Msg::Closed)
                // >
                //     <Flex scrollbar_y=true max_height="120px">
                //         {"Love is more than a word，it says so much. When I see these four letters, I almost feel your touch. This is only happened since，I fell in love with you. Why this word does this, I haven't got a clue."}
                //     </Flex>
                //     <Button
                //         slot="secondaryAction"
                //         text_button=true
                //         icon="close"
                //         dialog_action="close"
                //         label="关闭"
                //     />
                // </Dialog>
            </Animate>
        }
    }
}
