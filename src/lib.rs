#![recursion_limit = "256"]

use root::Root;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod markdown_preview_list;
mod markdown_visualizer;
mod request_loader;
mod root;
mod routes;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_as_body();
}
