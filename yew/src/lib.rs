use log::Level;
use wasm_bindgen::prelude::*;

mod app;
pub(crate) use app::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();

    yew::Renderer::<App>::new().render();
}
