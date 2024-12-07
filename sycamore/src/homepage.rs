use sycamore::prelude::*;
use wallet_adapter::WalletAdapter;

use crate::Footer;

#[component]
pub fn HomePage() -> View {
    // Import `WalletAdapter` anywhere like this
    let _adapter = use_context::<Signal<WalletAdapter>>();

    view! {
        div(class="flex-grow mx-4 lg:mx-auto"){
            div{
                div{
                    div(class="hero py-[64px]"){
                        div(class="hero-content text-center"){
                            div(class="max-w-2xl"){
                                h1(class="text-5xl font-bold"){"gm"}
                                p(class="py-6"){"Say hi to your new Solana dApp."}
                            }
                        }
                    }
                    div(class="max-w-xl mx-auto py-6 sm:px-6 lg:px-8 text-center"){
                        div(class="space-y-2"){
                            p{"Here are some helpful links to get you started."}
                            div{ a(href="https://docs.solana.com/", class="link", target="_blank",
                                    rel="noopener noreferrer"){"Solana Docs"}}
                            div{ a(href="https://faucet.solana.com/", class="link", target="_blank",
                                    rel="noopener noreferrer"){"Solana Faucet" }}
                            div{ a(href="https://solanacookbook.com/", class="link", target="_blank",
                                    rel="noopener noreferrer"){ "Solana Cookbook" }}
                            div{ a(href="https://solana.stackexchange.com/", class="link", target="_blank",
                                    rel="noopener noreferrer"){"Solana Stack Overflow" }}
                            div{ a(href="https://github.com/solana-developers/", class="link", target="_blank",
                                    rel="noopener noreferrer"){"Solana Developers GitHub" }}
                        }
                    }
                }
                div(style="position: fixed; z-index: 9999; inset: 16px; pointer-events: none;"){}

            }
            div(style="position: fixed; z-index: 9999; inset: 16px; pointer-events: none;"){}
        }
        (Footer())
    }
}
