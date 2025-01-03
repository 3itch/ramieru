#![allow(non_snake_case)]

use dioxus::prelude::*;
//use serde::{Deserialize, Serialize};
//use svg_attributes::cx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
static CSS: Asset = asset!("/assets/main.css");

pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div {
            id: "container",

            nav {
                class: "leftbar",
                ul {
                    button {
                        class: "btn code",
                    },

                    button {
                        class: "btn file",
                    },

                    button {
                        class: "btn settings",
                    },

                    button {
                        class: "btn execute",
                    },
                }
            },

            nav {
                class: "topbar",
                ul {
                    // li { "erm" }
                }
            }

            section {
                class: "code-editor",
            }

            section {
                class: "right",

                button {
                    class: "btn inject",
                }
            }
        }
    }
}
