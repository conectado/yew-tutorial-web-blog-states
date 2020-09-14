use crate::markdown_visualizer::view_markdown;
use anyhow::Error;
use http::{Request, Response};
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader {
    props: RequestLoaderProps,
    fetch_task: FetchTask,
    display_value: Option<Result<String, Error>>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}

pub enum FetchMessage {
    Loaded(Result<String, Error>),
}

impl Component for RequestLoader {
    type Properties = RequestLoaderProps;
    type Message = FetchMessage;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_task = fetch_url(&props.url, &link);
        RequestLoader {
            props,
            fetch_task,
            display_value: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let FetchMessage::Loaded(value) = msg;
        self.display_value = Some(value);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.display_value = None;
            self.props = props;
            self.fetch_task = fetch_url(&self.props.url, &self.link);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                match &self.display_value {
                    Some(response) => match response {
                        Ok(value) => view_markdown(value),
                        Error => html!{{"Error!"}},
                    },
                    None => html!{{"Loading..."}}
                }
            }
        }
    }
}

fn fetch_url(url: &str, link: &ComponentLink<RequestLoader>) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback = link.callback(|response: Response<Result<String, Error>>| {
        FetchMessage::Loaded(response.into_body())
    });

    FetchService::fetch(get_req, callback).unwrap()
}
