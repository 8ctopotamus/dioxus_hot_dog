use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
  rsx! {  
    header { "Welcome to hot dog" }
  }
}