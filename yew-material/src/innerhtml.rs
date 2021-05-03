use yew::prelude::*;
use yew_material_utils::set_inner_html;
use yewtil::NeqAssign;

use crate::comp_theme;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub html: String,
}

pub type This = InnerHtml;

comp_theme!(
    InnerHtml,
    |_, _| {},
    |_, _| {},
    |this: &This| {
        html! {
            <>
                { set_inner_html(&this.props.html) }
            </>
        }
    },
    "./doc/innerHtml.md"
);
