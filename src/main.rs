use dioxus::prelude::*;
use serde::Deserialize;
use reqwest;

fn main() {
  dioxus::launch(App)
}

static CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {	
  rsx! {
		document::Stylesheet { href: CSS }
		Title {}
		DogView {}
	}
}

#[component]
fn Title() -> Element {
	rsx! {
		div { id: "title",
			h1 { "🌭 HotDog " }
		}
	}
}

#[derive(Deserialize)]
struct DogApiResponse {
	message: String
}

#[component]
fn DogView() -> Element {
	let mut img_src = use_resource(|| async move {
		reqwest::get("https://dog.ceo/api/breeds/image/random")
			.await
			.unwrap()
			.json::<DogApiResponse>()
			.await
			.unwrap()
			.message
	});

	rsx! {
		div { id: "dogview",
			img {
				src: img_src.cloned().unwrap_or_default(),
				// alt: TODO
			}
		}
		div { id: "buttons",
			button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
			button { onclick: move |_| img_src.restart(), id: "save", "save!" }
		}
	}
}