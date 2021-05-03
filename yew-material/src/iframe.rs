use crate::comp_theme;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    #[prop_or_else(|| "100%".into())]
    pub width: String,
    #[prop_or_else(|| "0px".into())]
    pub height: String,
}

pub type This = Iframe;

comp_theme!(
    Iframe,
    |_, _| {},
    |_, _| {},
    |this: &This| {
        let mark = match this.props.src.contains("?") {
            true => "&",
            _ => "?",
        };
        let has_height = match this.props.height.as_str() {
            "0px" => "no",
            _ => "has",
        };
        html! {
            <div
                id=this.uuid
                style=format!("width:{};height:{};transition:height 0.3s;overflow: hidden;", this.props.width, this.props.height)
            >
                <iframe
                    style="border:none;width:100%;height:100%;top:0;left:0;"
                    src=format!("{}{}_services=iframe&_id={}&_height={}", this.props.src, mark, this.uuid, has_height)
                />
            </div>
        }
    },
    "./doc/iframe.md"
);
