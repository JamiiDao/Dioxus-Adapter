use dioxus::prelude::*;
use wallet_adapter::WalletAdapter;

pub fn Header(mut adapter: Signal<WalletAdapter>) -> Element {
    rsx! {
            div{id:"header",
                span {"DIOXUS DEMO"}
                span {
                    class:"menu-item",
                    a {id:"address", href:"#",
                        "data-wallet-address":adapter.read().connected_account().unwrap().address.as_str(),
                        "{adapter.read().connected_account().unwrap().shorten_address().unwrap()}"
                    }
                    ul{ class:"dropdown",
                        li{class:"dropdown-entry", a {
                            onclick: move |_| {
                                let address_element = adapter.read().document().get_element_by_id("address").unwrap();
                                let text_to_copy = address_element.get_attribute("data-wallet-address").unwrap();
                                let address_menu = adapter.read().document().get_element_by_id("copy-address").unwrap();
                                address_menu.set_text_content(Some("Copied"));
                                spawn(async move{
                                let writer_promise = adapter.read().window().navigator().clipboard().write_text(&text_to_copy);
                                wasm_bindgen_futures::JsFuture::from(writer_promise).await.unwrap();
                                let set_timeout = wasm_bindgen_futures::wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                    address_menu.set_text_content(Some("Copy Address"));
                                }) as Box<dyn Fn(_)>);

                                use wasm_bindgen_futures::wasm_bindgen::JsCast;
                                let set_timeout_fn = set_timeout.as_ref().dyn_ref::<wasm_bindgen_futures::js_sys::Function>().unwrap();
                                adapter.read().window()
                                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                                        set_timeout_fn,
                                        1000,
                                    )
                                    .expect("failed to set timeout");
                                    set_timeout.forget();
                                });
                            },
                        id:"copy-address",
                        href:"#",
                        "Copy Address"
                    }}
                    // li{class:"dropdown-entry", a {
                    //     onclick: move |_| {
                    //         show_modal.set(true);
                    //     },
                    //     href:"#",
                    //     "Change Wallet"
                    // }}
                    li{class:"dropdown-entry", a {
                        onclick: move |_| {
                            spawn(async move {
                                adapter.write().disconnect().await.unwrap();
                            });
                        },
                        href:"#",
                        "Disconnect"
                    }}
                }
            }
        }
    }
}