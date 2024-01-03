#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

#[derive(Props, PartialEq)]
pub struct PickerProps {
    #[props(into)]
    input: String,
    #[props(into)]
    button: String,
    #[props(into)]
    label: String,
    #[props(into)]
    files: UseRef<Files>,
    #[props(into)]
    selected: UseState<String>,
    #[props(into)]
    see_picker: UseState<bool>,
}

pub fn Picker(cx: Scope<PickerProps>) -> Element {
    // let window = dioxus_desktop::use_window(cx);
    cx.render(rsx!(
    div {
        class: "flex flex-row divide-y-0 p-4",
        label {
            r#for: "source_file",
            cx.props.label.as_str()
        }

        input {
            class: cx.props.input.as_str(),
            r#type: "text",
            id: "source_file",
            value: cx.props.selected.get().as_str(),
            size: "70",
            oninput: move |evt| {
                cx.props.selected.set(evt.data.value.clone());
            },
        }

        button {
            class: cx.props.button.as_str(),
            onclick: move |_| {
                tracing::info!("Submitted {}", cx.props.selected.get());
                cx.props.see_picker.set(!cx.props.see_picker.get());
            },
            "Select"
        }

        // button {
        //     class: cx.props.button.as_str(),
        //     onclick: move |_| {
        //         let dom = VirtualDom::new(Home);
        //         let size = dioxus_desktop::tao::dpi::Size::new(dioxus_desktop::tao::dpi::PhysicalSize::new(300, 450));
        //         window.new_window(dom, Config::new().with_window(
        //                 WindowBuilder::new()
        //                     .with_inner_size(size)
        //                     .with_always_on_top(true)
        //                     .with_title("Select source file location")
        //                 ));
        //     },
        //     "Submit"
        // }
    }

    div {
        class: "p-4",
        hidden: *cx.props.see_picker.get(),
        PickerContent { files: cx.props.files.clone() }
    }
        ))
}

#[component]
fn PickerContent(cx: Scope, files: UseRef<Files>) -> Element {
    cx.render(rsx!(
    p {
        format!("Target directory: {}", files.read().current().display())
    }
    files.read().contents.iter().enumerate().map(|(id, path)| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let icon_type = if file_name.contains('.') {
            rsx!(img {
                src: "../public/file.png",
                alt: "File",
                width: "16",
                onclick: move |_| {
                    files.write().enter(id);
                },
            })
        } else {
            rsx!(img {
                src: "../public/folder.png",
                alt: "Folder",
                width: "16",
                onclick: move |_| files.write().enter(id),
            })
        };
        rsx! (
            div {
                class: "flex flex-row mx-4 my-1",
                icon_type
            div {
                key: "{path.display()}",
                onclick: move |_| {
                    files.write().enter(id);
                },
                "{file_name}"
            }

            }
            )
    })
    button {
        onclick: move |_| files.write().up(),
        "←"
    }
    button {
        onclick: move |_| files.write().home(),
        "⌂"
    }
    button {
        onclick: move |_| files.write().forward(),
        "→"
    }
        ))
}
