use dioxus::prelude::*;

mod backend;
mod components;

use crate::components::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    #[route("/:..segments")]
    Player { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>,
}

fn use_music_player_provider() {
    let song = use_signal(|| "Stay The Night".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
fn App() -> Element {
    use_music_player_provider();
    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS}
        Router::<Route>{}
    }
}

#[component]
fn Player(segments: Vec<String>) -> Element {
    let song = use_context::<MusicPlayer>();
    rsx! {
        div {
            h3 { "Odd Playing {song.song}"}
            button {
                onclick: move |_| consume_context::<MusicPlayer>().song.set("Vienna".to_string()),
                "Shuffle Local"
            }

            button {
                onclick: move |_| *SONG.write() = "Beautiful".to_string(),
                "Shuffle Global"
            }
        }
    }
}
