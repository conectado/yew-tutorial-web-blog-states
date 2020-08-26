use markdown_visualizer::BlogDisplayerComponent;
use request_loader::RequestLoaderProps;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<BlogDisplayerComponent>::new().mount_as_body_with_props(RequestLoaderProps {
        url: "/articles/test.md".to_string(),
    });
}
