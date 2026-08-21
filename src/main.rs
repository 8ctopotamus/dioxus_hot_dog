use dioxus::prelude::*;

fn main() {
  dioxus::launch(App)
}

static CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
  	
	let breed = use_signal(|| "pitbull".to_string());
  
  rsx! {
		document::Stylesheet { href: CSS }
		div { id: "title",
			h1 { "🌭 HotDog " }
		}
		div { id: "dogview",
			img {
				src: "https://images.dog.ceo/breeds/{breed}/dog-3981540_1280.jpg",
				alt: breed,
			}
		}
		div { id: "buttons",
			button { id: "skip", "skip" }
			button { id: "save", "save!" }
		}
	}
}