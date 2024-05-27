<script lang="ts">
  import { stringToChars, type Char } from './lib/chars';
  import { onMount } from 'svelte';
  import _ from 'underscore';
  //@ts-expect-error: Don't have types for this lib
  import Keyboard from 'svelte-keyboard';

  let waiting = false;
  type Response = {
    stack: Array<string>;
    error: string;
  };
  let result: Response = {
    stack: [],
    error: ''
  };

  enum Mode {
    Edit,
    Run
  }

  let mode: Mode = Mode.Edit;

  // let code = `'(fn!\n  2 2 +\n12345678901234512345678901234561234567890123456\n'a def`;
  // let code = `0 'a def
  // '(fn
  //   1 'a def

  //   '(fn a)
  // )

  // call
  // call
  // a`;
  let code = `"hello, "\n"world"\nconcat`;

  function save_code() {
    localStorage.setItem('code', code);
  }

  $: code_with_space = (() => {
    if (mode === Mode.Edit) {
      return code.endsWith(' ') ? code : `${code} `;
    } else {
      if (waiting) {
        return ' ';
      } else {
        if (result.error) {
          return `Err: ${result.error}`;
        } else {
          let stack = result.stack.join(', ');
          return stack.endsWith(' ') ? stack : `${stack} `;
        }
      }
    }
  })();
  $: chars = stringToChars(code_with_space, mode === Mode.Edit ? 15 : 16);
  $: line_offset = {
    [Mode.Edit]: 0,
    [Mode.Run]: 0
  };
  $: buffer_window = (() => {
    let start = charAtLineStart(line_offset[mode]);
    return chars.slice(start?.index ?? 0);
  })();

  function charAtCursor(): Char | undefined {
    return chars.find((char) => char.index === cursor[mode]);
  }

  function charAtLineStart(line: number): Char | undefined {
    return chars.find((char) => char.line === line);
  }

  function charAtLineEnd(line: number): Char | undefined {
    return chars
      .slice()
      .reverse()
      .find((char) => char.line === line);
  }

  async function execute() {
    waiting = true;

    let res = await fetch(`http://${document.location.host}/execute`, {
      method: 'POST',
      body: code
    });

    result = await res.json();
    waiting = false;
  }

  let cursor = {
    [Mode.Edit]: 0,
    [Mode.Run]: 0
  };
  let canvas: HTMLCanvasElement | null = null;

  $: (() => {
    if (!canvas) return;

    const height = canvas.height;
    const width = canvas.width;

    const c = canvas.getContext('2d');
    if (!c) return;

    c.strokeStyle = 'black';
    c.fillStyle = 'black';

    c.fillRect(0, 0, width, height);

    c.fillStyle = 'white';
    c.font = '24px monospace';

    let wrapping = false;
    for (let char of buffer_window) {
      wrapping = char.wrapped;

      let x = char.line_index;
      let y = char.line - line_offset[mode];

      let x_tile = 14.4;
      let y_tile = 20.1;

      if (!wrapping && mode !== Mode.Run) {
        if (x === 0) c.fillText(':', x * x_tile, (y + 1) * y_tile);
        x += 1;
      }

      if (char.index === cursor[mode]) {
        c.fillStyle = 'white';
        c.fillRect(x * x_tile, y * y_tile + 2, x_tile, y_tile + 2);
        c.fillStyle = 'black';
      }

      c.fillText(char.char, x * x_tile, (y + 1) * y_tile);
      c.fillStyle = 'white';
    }
  })();

  function clamp_cursor() {
    cursor[mode] = Math.max(0, Math.min(cursor[mode], code_with_space.length - 1));
  }

  function move(string: string) {
    navigator.vibrate(50);

    let current_char = charAtCursor();
    let current_line = current_char?.line ?? 0;

    if (string === 'ArrowLeft') {
      cursor[mode] -= 1;
    } else if (string === 'ArrowRight') {
      cursor[mode] += 1;
    } else if (string === 'ArrowUp') {
      let next_line = charAtLineStart(current_line - 1);
      let next_line_end = charAtLineEnd(current_line - 1);
      if (current_char && next_line && next_line_end) {
        cursor[mode] = Math.min(next_line.index + current_char.line_index, next_line_end.index);
      } else {
        cursor[mode] = 0;
      }
    } else if (string === 'ArrowDown') {
      let next_line = charAtLineStart(current_line + 1);
      let next_line_end = charAtLineEnd(current_line + 1);
      if (current_char && next_line && next_line_end) {
        cursor[mode] = Math.min(next_line.index + current_char.line_index, next_line_end.index);
      } else {
        cursor[mode] = code_with_space.length;
      }
    } else if (string === 'Home') {
      let next_line = charAtLineStart(current_line);
      if (next_line) {
        cursor[mode] = next_line.index;
      }
    } else if (string === 'End') {
      let next_line = charAtLineEnd(current_line);
      if (next_line) {
        cursor[mode] = next_line.index;
      }
    } else if (string === 'Backspace' || string === 'Delete') {
      let splice = code.split('');
      splice.splice(cursor[mode], 1);
      code = splice.join('');
      cursor[mode] -= 1;
    }

    clamp_cursor();
    save_code();

    let new_current_char = charAtCursor();
    let new_current_line = new_current_char?.line ?? 0;
    if (new_current_line !== current_line) {
      if (new_current_line < line_offset[mode]) {
        // Cursor decreased
        line_offset[mode] = Math.max(new_current_line, 0);
      } else if (new_current_line >= line_offset[mode] + 7) {
        // Cursor increased
        line_offset[mode] = Math.max(new_current_line - 7 + 1, 0);
      }
    }
  }

  function write(string: string) {
    navigator.vibrate(50);

    if (mode !== Mode.Edit) return;

    if (string === 'Enter') {
      code = code.slice(0, cursor[mode]) + '\n' + code.slice(cursor[mode]);
      cursor[mode] += 1;
    }

    if (string.length === 1) {
      code = code.slice(0, cursor[mode]) + string + code.slice(cursor[mode]);
      cursor[mode] += 1;
    }

    save_code();
  }

  const keydown = (e: KeyboardEvent) => {
    move(e.key);
  };

  const keypress = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      e.preventDefault();
      execute();
      return;
    }

    write(e.key);
  };

  const virtual_keydown = (e: CustomEvent) => {
    if (e.detail === 'Space') {
      write(' ');
    } else if (e.detail === 'Backspace') {
      move(e.detail);
    } else {
      write(e.detail);
    }
  };

  onMount(() => {
    let storage = localStorage.getItem('code');
    if (storage !== null) {
      code = storage;
    }

    document.addEventListener('keydown', keydown);
    document.addEventListener('keypress', keypress);
  });

  function switch_mode() {
    if (mode === Mode.Run) {
      mode = Mode.Edit;
    } else if (mode === Mode.Edit) {
      execute();
      mode = Mode.Run;
      cursor[mode] = 0;
      line_offset[mode] = 0;
    }
  }
</script>

<div id="device" class="col">
  <canvas id="buffer" bind:this={canvas} width="231px" height="143px"></canvas>
  <div id="button-grid">
    <div id="function-grid">
      <button on:click={() => switch_mode()} id="home">Run/Edit</button>
    </div>
    <div id="navigation-grid">
      <button on:click={() => move('Home')} id="home">Home</button>
      <button on:click={() => move('ArrowUp')} id="up">Up</button>
      <button on:click={() => move('End')} id="end">End</button>
      <button on:click={() => move('ArrowLeft')} id="left">Left</button>
      <button on:click={() => write('Enter')} id="enter">Enter</button>
      <button on:click={() => move('ArrowRight')} id="right">Right</button>
      <button on:click={() => move('ArrowDown')} id="down">Down</button>
    </div>
  </div>
  <div class="keyboard-container">
    <Keyboard on:keydown={virtual_keydown} />
  </div>
</div>

<style lang="scss">
  .col {
    display: flex;
    flex-direction: column;
  }

  #device {
    width: min-content;

    button {
      width: 50px;
      height: 50px;
    }

    #button-grid {
      display: flex;
      flex-direction: row;
      justify-content: space-around;

      #navigation-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-rows: 1fr 1fr 1fr;
        align-self: flex-end;

        #down {
          grid-column-start: 2;
        }
      }
    }
  }
</style>
