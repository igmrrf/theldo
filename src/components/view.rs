use dioxus::prelude::*;

use crate::backend::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

async fn get_dog_src() -> String {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<DogApi>()
        .await
        .unwrap();
    response.message
}

#[component]
pub fn DogView() -> Element {
    let mut dog_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    let skip = move |_| async move {
        let message = get_dog_src().await;
        println!("{}", message);
        dog_src.set(Some(message.to_string()))
    };

    let save = move |_| async move {
        let current = dog_src.cloned().unwrap();
        dog_src.restart();
        _ = save_dog(current).await;
    };

    rsx! {
        div {
            id: "dogview",
            img { src: dog_src.cloned().unwrap_or_default(), max_height: "200px"}
        }
        div {
            id: "buttons",
            button { onclick: skip, id: "skip", "skip"}
            button { onclick: save, id: "save", "save!"}

        }

    }
}
