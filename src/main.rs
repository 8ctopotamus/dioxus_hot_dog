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
	let image_src = use_signal(|| "".to_string());

	// let skip = move |evt| {};
	let save = move |_| async move {
		let response = reqwest::get()
			.await
			.unwrap()
			.json::<DogApiResponse>()
			.await
		image_src.set(response.unwrap().message);
	};

	rsx! {
		div { id: "dogview",
			img {
				src: "{image_src}",
				// alt: breed,
			}
		}
		div { id: "buttons",
			// button { onclick: skip, id: "skip", "skip" }
			button { onclick: save, id: "save", "save!" }
		}
	}
}