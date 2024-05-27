<script lang="ts">
  import { stringToChars, type Char } from '$lib/chars';
  import { onMount } from 'svelte';
  import _ from 'underscore';

  let waiting = false;
  type Response = {
    stack: Array<string>;
    error: string;
  };
  let result: Response = {
    stack: [],
    error: ''
  };

  // let code = `'(fn!\n  2 2 +\n12345678901234512345678901234561234567890123456\n'a def`;
  let code = `'(fn!\n  2 2 +\n`;
  $: code_with_space = code.endsWith(' ') ? code : `${code} `;
  $: chars = stringToChars(code_with_space);

  function charAtCursor(): Char | undefined {
    return chars.find((char) => char.index === cursor);
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

    let res = await fetch('http://127.0.0.1:7777/execute', {
      method: 'POST',
      body: code
    });

    result = await res.json();
    waiting = false;
  }

  let cursor = 0;
  let line_offset = 0;
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
    for (let char of chars) {
      wrapping = char.wrapped;

      let x = char.line_index;
      let y = char.line;

      if (!wrapping) {
        if (x === 0) c.fillText(':', x * 14.4, (y + 1) * 20);
        x += 1;
      }

      if (char.index === cursor) {
        c.fillStyle = 'white';
        c.fillRect(x * 14.4, y * 20 + 2, 14.4, 22);
        c.fillStyle = 'black';
      }

      c.fillText(char.char, x * 14.4, (y + 1) * 20);
      c.fillStyle = 'white';
    }
  })();

  function move(string: string) {
    let current_char = charAtCursor();
    let current_line = current_char?.line ?? 0;

    if (string === 'ArrowLeft') {
      cursor -= 1;
    } else if (string === 'ArrowRight') {
      cursor += 1;
    } else if (string === 'ArrowUp') {
      let next_line = charAtLineStart(current_line - 1);
      let next_line_end = charAtLineEnd(current_line - 1);
      if (current_char && next_line && next_line_end) {
        cursor = Math.min(next_line.index + current_char.line_index, next_line_end.index);
      } else {
        cursor = 0;
      }
    } else if (string === 'ArrowDown') {
      let next_line = charAtLineStart(current_line + 1);
      let next_line_end = charAtLineEnd(current_line + 1);
      if (current_char && next_line && next_line_end) {
        cursor = Math.min(next_line.index + current_char.line_index, next_line_end.index);
      } else {
        cursor = code_with_space.length;
      }
    } else if (string === 'Home') {
      let next_line = charAtLineStart(current_line);
      if (next_line) {
        cursor = next_line.index;
      }
    } else if (string === 'End') {
      let next_line = charAtLineEnd(current_line);
      if (next_line) {
        cursor = next_line.index;
      }
    } else if (string === 'Backspace' || string === 'Delete') {
      let splice = code.split('');
      splice.splice(cursor, 1);
      code = splice.join('');
    }

    console.log({ code, code_with_space });

    cursor = Math.max(0, Math.min(cursor, code_with_space.length - 1));
  }

  function write(string: string) {
    if (string === 'Enter') {
      code = code.slice(0, cursor) + '\n' + code.slice(cursor);
      cursor += 1;
    }

    if (string.length === 1) {
      code = code.slice(0, cursor) + string + code.slice(cursor);
      cursor += 1;
    }
  }

  onMount(() => {
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

    document.addEventListener('keydown', keydown);
    document.addEventListener('keypress', keypress);
  });
</script>

{#if waiting}
  <pre>executing...</pre>
{:else if result.error}
  <pre>error: {result.error}</pre>
{:else}
  <pre>stack: {result.stack.join(', ')}</pre>
{/if}

<div id="device" class="col">
  <canvas id="buffer" bind:this={canvas} width="231px" height="143px"></canvas>
  <div id="button-grid">
    <div id="navigation-grid">
      <button on:click={() => move('ArrowUp')} id="up">Up</button>
      <button on:click={() => move('ArrowLeft')} id="left">Left</button>
      <button on:click={() => write('Enter')} id="enter">Enter</button>
      <button on:click={() => move('ArrowRight')} id="right">Right</button>
      <button on:click={() => move('ArrowDown')} id="down">Down</button>
    </div>
    <button on:click={() => move('Backspace')} id="delete">Del</button>
  </div>
</div>

<style lang="scss">
  .row {
    display: flex;
    flex-direction: row;
  }

  .col {
    display: flex;
    flex-direction: column;
  }

  .center {
    justify-content: center;
  }

  #device {
    width: min-content;

    button {
      width: 50px;
      height: 50px;
    }

    #button-grid {
      #navigation-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-rows: 1fr 1fr 1fr;

        width: min-content;

        #up {
          grid-column-start: 2;
        }

        #left {
          grid-row-start: 2;
        }

        #enter {
          grid-row-start: 2;
        }

        #right {
          grid-row-start: 2;
        }

        #down {
          grid-column-start: 2;
        }
      }
    }
  }
</style>
