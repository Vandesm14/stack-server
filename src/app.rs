use std::ops::Deref;

use trunk_template::{Editor, MoveAction};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  let editor = use_state(Editor::default);

  let move_evt = |action: MoveAction| {
    let editor = editor.clone();

    Callback::from(move |_| {
      let mut new_editor = editor.deref().clone();
      new_editor.navigate(action);
      editor.set(new_editor)
    })
  };

  html! {
    <main>
      <p>{editor.cursor}</p>
      <div id="device" class="col">
        <canvas id="buffer" width="231px" height="143px"></canvas>
        <div id="button-grid">
          <div id="function-grid">
            <button id="mode" onclick={move_evt(MoveAction::Mode)}>{"Run/Edit"}</button>
          </div>
          <div id="navigation-grid">
            <button id="home" onclick={move_evt(MoveAction::Home)}>{"Home"}</button>
            <button id="up" onclick={move_evt(MoveAction::Up)}>{"Up"}</button>
            <button id="end" onclick={move_evt(MoveAction::End)}>{"End"}</button>
            <button id="left" onclick={move_evt(MoveAction::Left)}>{"Left"}</button>
            <button id="enter" onclick={move_evt(MoveAction::Enter)}>{"Enter"}</button>
            <button id="right" onclick={move_evt(MoveAction::Right)}>{"Right"}</button>
            <button id="down" onclick={move_evt(MoveAction::Down)}>{"Down"}</button>
          </div>
        </div>
        <div class="keyboard-container"></div>
      </div>
    </main>
  }
}
