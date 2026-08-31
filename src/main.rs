use dioxus::prelude::*;

mod backend;
mod components;

use crate::components::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)]
	#[route("/")]
	DogView,
	
	#[route("/favorites")]
	Favorites,
	
	// segments catch-all example
	// #[route("/:...segements")]
	// PageNotFound { segments: Vec<String> },
}

#[component]
fn App() -> Element {	
  rsx! {
		document::Stylesheet { href: asset!("/assets/main.css") }
		Router::<Route> {}
	}
}

fn main() {
  dioxus::launch(App)
}