use dioxus::prelude::*;

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

#[component]
fn DogView() -> Element {
	let image_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

	let skip = move |evt| {};
	let save = move |evt| {};

	rsx! {
		div { id: "dogview",
			img {
				src: "{image_src}",
				// alt: breed,
			}
		}
		div { id: "buttons",
			button { onclick: skip, id: "skip", "skip" }
			button { onclick: save, id: "save", "save!" }
		}
	}
}