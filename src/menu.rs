#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

pub fn Menu(cx: Scope) -> Element {
    tracing::info!("Menu drawing.");
    cx.render(rsx!(
        div {
            class: "flex flex-row justify-between",
            img {
                class: "p-3",
                src: "../public/gp_logo.png",
                alt: "City of Grants Pass Logo",
                width: "60",
            }
            p {
                class: "self-center text-lg",
                "Mailing List Client" }
            DarkModeButton {
                on_click: move |_| {
                    let theme = use_shared_state::<Theme>(cx);
                    let mut msg = "".to_string();
                    match theme {
                        Some(value) => {
                            let t = value.read().clone();
                            *value.write() = t.next();
                        }
                        None => msg.push_str("No theme found."),
                    }
                },
            }
        }
    ))
}
