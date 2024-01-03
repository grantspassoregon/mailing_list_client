#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::tao::window::Icon;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::prelude::*;
use image::DynamicImage;
use mailing_list_client::prelude::*;
use tracing::info;

fn main() -> ClientResult<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    info!("Subscriber initialized.");
    // let icon = std::fs::read("./public/gp_logo.png")?;
    let icon = image::io::Reader::open("./public/gp_logo.png")?
        .decode()?
        .into_bytes();
    let icon = Icon::from_rgba(icon, 200, 267)?;
    // dioxus_desktop::launch(App);
    dioxus_desktop::launch_cfg(
        App,
        Config::new().with_window(
            WindowBuilder::new()
                .with_title("Mailing List Client")
                .with_window_icon(Some(icon)),
        ),
    );
    info!("🚀 Client started successfully");
    Ok(())
}

fn App(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    if theme.is_none() {
        use_shared_state_provider(cx, || Theme::Light);
        info!("Theme set to light.");
    }
    cx.render(rsx! {
        style { include_str!("../public/tailwind.css") }
        Router::<Route> {}
    })
}

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/home")]
    // #[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
    #[redirect("", || Route::Home {})]
    Home {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
