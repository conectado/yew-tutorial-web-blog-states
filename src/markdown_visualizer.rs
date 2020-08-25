use pulldown_cmark as pc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys;

pub struct MarkdownVisualizer;

impl Component for MarkdownVisualizer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MarkdownVisualizer
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            {view_markdown("# This is bold!")}
        }
    }
}

fn create_markdown_container() -> web_sys::Element {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown-body");
    div
}

fn view_markdown(value: &str) -> Html {
    let parser = pc::Parser::new(value);
    let mut html_output = String::new();
    pc::html::push_html(&mut html_output, parser);

    let div = create_markdown_container();

    div.set_inner_html(&html_output);

    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}
