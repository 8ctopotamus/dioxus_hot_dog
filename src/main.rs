use dioxus::prelude::*;
use serde::Deserialize;
use reqwest;

mod components;
mod backend;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)]
	#[route("/")]
	DogView,
	
	#[route("/favorites")]
	Favorites,
	
	// #[route("/:...segements")]
	// PageNotFound { segments: Vec<String> },
}

#[component]
fn App() -> Element {	
  rsx! {
		document::Stylesheet { href: CSS }
		Router::<Route> {}
	}
}

fn main() {
  dioxus::launch(App)
}



#[derive(Deserialize)]
struct DogApiResponse {
	message: String
}

#[component]
fn DogView() -> Element {
	rsx! {"hi"}
	// let mut img_src = use_resource(|| async move {
	// 	reqwest::get("https://dog.ceo/api/breeds/image/random")
	// 		.await
	// 		.unwrap()
	// 		.json::<DogApiResponse>()
	// 		.await
	// 		.unwrap()
	// 		.message
	// });

	// rsx! {
	// 	div { id: "dogview",
	// 		img {
	// 			src: img_src.cloned().unwrap_or_default(),
	// 			// alt: TODO
	// 		}
	// 	}
	// 	div { id: "buttons",
	// 		button { 
	// 			id: "skip",
	// 			onclick: move |_| img_src.restart(), 
	// 			"skip" 
	// 		}
	// 		button { 
	// 			id: "save", 
	// 			onclick: move |_| async move {
	// 				let current = img_src.cloned().unwrap();
	// 				img_src.restart();
	// 				let _ = save_dog(current).await;
	// 			},
	// 			"save!" 
	// 		}
	// 	}
	// }
}