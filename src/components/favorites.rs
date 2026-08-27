use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
  rsx! {
    let mut favorites = use_server_future(super::backend::list_dogs)?;
    
    rsx! {
      div {
        id: "favorites",
        div {
          id: "favorites-container",
          for (id, url) in favorites().unwrap().unwrap() {
            div {
              key: "{id}",
              class: "favorite-dog",
              img { src: "{url}" }
            }
          }
        }
      }
    }
  }
}