#![recursion_limit = "256"]

use markdown_preview_list::BlogPreviewListDisplayerComponent;
use request_loader::RequestLoaderProps;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod markdown_preview_list;
mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<BlogPreviewListDisplayerComponent>::new().mount_as_body_with_props(RequestLoaderProps {
        url: constants::ARTICLE_LIST_URI.to_string(),
    });
}
