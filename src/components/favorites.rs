use dioxus::prelude::*;

use crate::backend::{delete_dog, list_dogs};

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_server_future(list_dogs)?;
    let remove = move |id| async move {
        _ = delete_dog(id).await;
        favorites.restart();
    };

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id, url) in favorites().unwrap().unwrap(){
                    div {
                        key: "{id}",
                        class: "favorite-dog",
                        img { src: "{url}" }
                        button { onclick: move |_evnt| remove(id), id: "close-button", "🧨"}
                    }

                }

            }

        }
    }
}
