use wasm_bindgen::prelude::*;

mod app;
pub(crate) use app::*;

#[wasm_bindgen(start)]
pub fn main() {

    yew::Renderer::<App>::new().render();
}
