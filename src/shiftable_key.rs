use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShiftableKeyProps {
  pub top: String,
  pub bottom: String,
  pub onclick: Callback<MouseEvent>,
}

#[function_component(ShiftableKey)]
pub fn shiftable_key(props: &ShiftableKeyProps) -> Html {
  html! {
    <button class="shiftable-key" onclick={&props.onclick}>
      <span class="top">{&props.top}</span>
      <span class="bottom">{&props.bottom}</span>
    </button>
  }
}
