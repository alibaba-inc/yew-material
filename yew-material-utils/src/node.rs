use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

///创建钩子节点时对应的html标签
pub static HOOK_NODE_TAG: &str = "yew-mdc";
///钩子节点的属性名，用于将yew组件节点挂载到html节点上
pub static HOOK_NODE_ROOT: &str = "parent";

///钩子节点初始化方法
pub fn default(name: &str, key: &str) -> Node {
    let node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&HOOK_NODE_TAG)
        .unwrap();
    node.set_attribute(&format!("{}-{}", HOOK_NODE_ROOT, name), key)
        .unwrap();
    node.set_attribute("style", "display: contents").unwrap();
    Node::from(node)
}

pub enum Msg {}

pub struct YNode {
    props: YProps,
}

///mount_with_props对应的动态节点
#[derive(Clone, PartialEq, Properties)]
pub struct YProps {
    pub node: VNode,
}

impl Component for YNode {
    type Message = Msg;
    type Properties = YProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn rendered(&mut self, _: bool) {
        //
    }

    fn view(&self) -> Html {
        html! {self.props.node.clone()}
    }
}
