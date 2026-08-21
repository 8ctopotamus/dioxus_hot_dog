use dioxus::prelude::*;

mod components;

use components::header::Header;
use components::footer::Footer;
use components::dog_app::DogApp;

fn main() {
  dioxus::launch(App)
}

#[component]
fn App() -> Element {
  	
	let dog_name = use_signal(|| "Black Lab".to_string());
  
  rsx! {
		Header {}
		DogApp { breed: dog_name }
		Footer {}
  }
}