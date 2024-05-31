use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <main>
      <div id="device" class="col">
        <canvas id="buffer" width="231px" height="143px"></canvas>
        <div id="button-grid">
          <div id="function-grid">
            <button id="home">{"Run/Edit"}</button>
          </div>
          <div id="navigation-grid">
            <button id="home">{"Home"}</button>
            <button id="up">{"Up"}</button>
            <button id="end">{"End"}</button>
            <button id="left">{"Left"}</button>
            <button id="enter">{"Enter"}</button>
            <button id="right">{"Right"}</button>
            <button id="down">{"Down"}</button>
          </div>
        </div>
        <div class="keyboard-container"></div>
      </div>
    </main>
  }
}
