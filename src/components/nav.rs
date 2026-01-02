use crate::Route;
use dioxus::prelude::*;

const ICON: Asset = asset!("/assets/header.svg");

#[derive(Clone)]
struct TitleState(String);

#[component]
pub fn NavBar() -> Element {
    use_context_provider(|| TitleState("The Lazy".to_string()));
    let title = use_context::<TitleState>();
    rsx! {
        img { src: ICON, max_height: "100px"}
        div {
            id: "title",
            Link { to: Route::DogView,
                h1 { "{title.0}!   💻"}
            }

            Link { to: Route::Favorites, id: "heart", "♥️" }
        }
        Outlet::<Route>{}
    }
}
