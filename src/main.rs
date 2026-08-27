use dioxus::prelude::*;
use serde::Deserialize;
use reqwest;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
  dioxus::launch(App)
}

#[post("/api/save_dog")]
async fn save_dog(image: String) -> Result<()> {
	use std::io::Write;

	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open("dogs.txt")
		.unwrap();

	file.write_fmt(format_args!("{image}\n"));

	Ok(())
}

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
			button { 
				id: "skip",
				onclick: move |_| img_src.restart(), 
				"skip" 
			}
			button { 
				id: "save", 
				onclick: move |_| async move {
					let current = img_src.cloned().unwrap();
					img_src.restart();
					let _ = save_dog(current).await;
				},
				"save!" 
			}
		}
	}
}