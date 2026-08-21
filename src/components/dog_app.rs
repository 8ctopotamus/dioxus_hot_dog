use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DogAppProps {
  pub breed: String,
}

#[component]
pub fn DogApp(props: DogAppProps) -> Element {
  rsx! {
    "Breed: {props.breed}"
  }
}