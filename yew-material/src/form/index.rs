use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;
use yew::prelude::*;
use yew::Callback;
use yew_material_utils::{form_traversing, CallbackRes};

pub type ValidityRes = CallbackRes<String, bool>;

pub trait FormRes: Serialize + DeserializeOwned + Clone + 'static {
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Res<T> {
    valid: bool,
    values: T,
}

pub enum Msg {}

pub struct Form<T>
where
    T: FormRes,
{
    id: String,
    props: Props<T>,
}

#[derive(Clone, Properties)]
pub struct Props<T>
where
    T: FormRes,
{
    pub children: Children,
    #[prop_or_default]
    pub onsubmit: Callback<T>,
}

impl<T> Component for Form<T>
where
    T: FormRes,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let id = self.id.to_owned();
        let onsubmit = self.props.onsubmit.to_owned();

        html! {
            <form id=&id onsubmit=Callback::from(move |e: web_sys::FocusEvent| {
                e.prevent_default();
                let res = form_traversing(&id).into_serde::<Res<T>>().unwrap();
                if res.valid {
                    onsubmit.emit(res.values);
                }
            })>
                {self.props.children.to_owned()}
            </form>
        }
    }
}
