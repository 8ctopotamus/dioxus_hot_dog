use dioxus::prelude::*;

mod backend;
mod components;

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