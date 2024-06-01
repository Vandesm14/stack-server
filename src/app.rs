use std::ops::Deref;

use trunk_template::{Editor, EditorMode, MoveAction};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  let canvas_ref = use_node_ref();
  let editor = use_state(|| Editor::new().with_code("2 2 +\nh".to_owned()));

  let move_evt = |action: MoveAction| {
    let editor = editor.clone();

    Callback::from(move |_| {
      let mut new_editor = editor.deref().clone();
      new_editor.navigate(action);
      editor.set(new_editor);
    })
  };

  let write_evt = |char: char| {
    let editor = editor.clone();

    Callback::from(move |_| {
      let mut new_editor = editor.deref().clone();
      new_editor.write(char);
      editor.set(new_editor);
    })
  };

  {
    let canvas_ref = canvas_ref.clone();
    let editor = editor.clone();

    use_effect_with_deps(
      |(canvas_ref, editor)| {
        let canvas = canvas_ref
          .cast::<HtmlCanvasElement>()
          .expect("canvas_ref not attached to canvas element");

        let context = canvas
          .get_context("2d")
          .unwrap()
          .unwrap()
          .dyn_into::<web_sys::CanvasRenderingContext2d>()
          .unwrap();

        context.set_stroke_style(&JsValue::from_str("black"));
        context.set_fill_style(&JsValue::from_str("black"));

        context.fill_rect(
          0.0,
          0.0,
          canvas.width() as f64,
          canvas.height() as f64,
        );

        context.set_fill_style(&JsValue::from_str("white"));
        context.set_font("24px monospace");

        let mut wrapping = false;
        for char in editor.chars_window() {
          wrapping = char.wrapped;

          let x = char.line_index;
          let y = char.line.saturating_sub(editor.line_offset);

          let mut x = x as f64;
          let y = y as f64;

          let x_tile = 14.4;
          let y_tile = 20.1;

          if !wrapping && editor.mode != EditorMode::Run {
            if x == 0.0 {
              context
                .fill_text(&':'.to_string(), x * x_tile, (y + 1.0) * y_tile)
                .unwrap();
            }
            x += 1.0;
          }

          if char.index == editor.cursor {
            context.fill_rect(x * x_tile, y * y_tile + 2.0, -2.0, y_tile + 2.0)
          }

          context
            .fill_text(&char.char.to_string(), x * x_tile, (y + 1.0) * y_tile)
            .unwrap();
        }

        move || {}
      },
      (canvas_ref, editor),
    );
  }

  html! {
    <main>
      <div id="device">
        <canvas id="buffer" width="231px" height="143px" ref={canvas_ref}></canvas>
        <div id="button-grid">
          <div id="navigation-row">
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
          <div id="keyboard-grid">
            // Row
            <button onclick={write_evt('1')}>{"!"}</button>
            <button onclick={write_evt('2')}>{"%"}</button>
            <button onclick={write_evt('3')}>{"+"}</button>
            <button onclick={write_evt('4')}>{"-"}</button>
            <button onclick={write_evt('5')}>{"*"}</button>
            <button onclick={write_evt('6')}>{"/"}</button>
            <button onclick={write_evt('7')}>{"<"}</button>
            <button onclick={write_evt('8')}>{">"}</button>
            <button onclick={write_evt('9')}>{"="}</button>
            <button onclick={write_evt('0')}>{"'"}</button>

            // Row
            <button onclick={write_evt('1')}>{"1"}</button>
            <button onclick={write_evt('2')}>{"2"}</button>
            <button onclick={write_evt('3')}>{"3"}</button>
            <button onclick={write_evt('4')}>{"4"}</button>
            <button onclick={write_evt('5')}>{"5"}</button>
            <button onclick={write_evt('6')}>{"6"}</button>
            <button onclick={write_evt('7')}>{"7"}</button>
            <button onclick={write_evt('8')}>{"8"}</button>
            <button onclick={write_evt('9')}>{"9"}</button>
            <button onclick={write_evt('0')}>{"0"}</button>

            // Row
            <button onclick={write_evt('q')}>{"q"}</button>
            <button onclick={write_evt('w')}>{"w"}</button>
            <button onclick={write_evt('e')}>{"e"}</button>
            <button onclick={write_evt('r')}>{"r"}</button>
            <button onclick={write_evt('t')}>{"t"}</button>
            <button onclick={write_evt('y')}>{"y"}</button>
            <button onclick={write_evt('u')}>{"u"}</button>
            <button onclick={write_evt('i')}>{"i"}</button>
            <button onclick={write_evt('o')}>{"o"}</button>
            <button onclick={write_evt('p')}>{"p"}</button>

            // Row
            <button onclick={write_evt('a')}>{"a"}</button>
            <button onclick={write_evt('s')}>{"s"}</button>
            <button onclick={write_evt('d')}>{"d"}</button>
            <button onclick={write_evt('f')}>{"f"}</button>
            <button onclick={write_evt('g')}>{"g"}</button>
            <button onclick={write_evt('h')}>{"h"}</button>
            <button onclick={write_evt('j')}>{"j"}</button>
            <button onclick={write_evt('k')}>{"k"}</button>
            <button onclick={write_evt('l')}>{"l"}</button>
            <button class="dark" onclick={move_evt(MoveAction::Enter)}>{"ret"}</button>

            // Row
            <button onclick={write_evt('z')}>{"z"}</button>
            <button onclick={write_evt('x')}>{"x"}</button>
            <button onclick={write_evt('c')}>{"c"}</button>
            <button onclick={write_evt('v')}>{"v"}</button>
            <button onclick={write_evt('b')}>{"b"}</button>
            <button onclick={write_evt('n')}>{"n"}</button>
            <button onclick={write_evt('m')}>{"m"}</button>
            <button class="dark" onclick={write_evt(' ')}>{"spc"}</button>
            <button class="dark" onclick={move_evt(MoveAction::Delete)}>{"del"}</button>
            <button class="dark">{"shift"}</button>
          </div>
        </div>
      </div>
    </main>
  }
}
