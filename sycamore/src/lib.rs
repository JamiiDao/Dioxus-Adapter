use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use wallet_adapter::WalletAdapter;
use wasm_bindgen::prelude::*;

mod header;
use header::*;

mod homepage;
use homepage::*;

mod footer;
pub(crate) use footer::*;

#[wasm_bindgen(start)]
pub fn main() {
    sycamore::render(App);
}

#[component]
fn App() -> View {
    let adapter = WalletAdapter::init().unwrap();

    provide_context(create_signal(adapter));

    view! {
        div(id="main") {
            Router(
                integration=HistoryIntegration::new(),
                view=|route: ReadSignal<AppRoutes>| {

                    view! {
                        div(id="main") {
                            (Header())
                            (match route.get_clone() {
                                AppRoutes::Home => HomePage(),
                                AppRoutes::NotFound => view! {
                                    h1 { "Not Found" }
                                },
                            })
                        }
                    }
                }
            )
        }
    }
}

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[not_found]
    NotFound,
}