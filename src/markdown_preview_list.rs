use crate::article_list::Articles;
use crate::markdown_visualizer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;

pub type BlogPreviewListDisplayerComponent =
    RequestLoader<BlogPreviewListDisplayer, Json<Result<Articles, Error>>>;

pub struct BlogPreviewListDisplayer;

impl Displayer<Json<Result<Articles, Error>>> for BlogPreviewListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> Html {
        match text {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {
                        {
                            for arts.articles.iter().map(|item| {
                                html!{
                                    <div class="container rounded bg-dark" style="margin-top: 1%; -webkit-line-clamp: 9; overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical;">
                                        <div style="display: block; overflow: hidden; ">
                                            <BlogDisplayerComponent  url={("/articles/".to_string() + item)} />
                                        </div>
                                    </div>
                                }
                            })
                        }
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => html! {<p>{"Loading"}</p>},
        }
    }
}
