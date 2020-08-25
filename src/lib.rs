use markdown_visualizer::MarkdownVisualizer;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod markdown_visualizer;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MarkdownVisualizer>::new().mount_to_body();
}
