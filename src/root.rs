use crate::constants;
use crate::markdown_preview_list::BlogPreviewListDisplayerComponent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Root;

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="text-white" style="overflow: auto; position: fixed; height: 100%; width: 100%; background-color: black;">
                        <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI/>
                </div>
            </body>
        }
    }
}
