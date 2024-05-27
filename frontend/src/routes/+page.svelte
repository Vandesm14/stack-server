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
  $: chars = stringToChars(code);

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
    // let closest_line = 0;
    // for (let i in indicies) {
    //   if (indicies[i].index <= cursor) {
    //     closest_line = parseInt(i);
    //   } else break;
    // }
    // let current_line = indicies[closest_line];
    // let offset = cursor - current_line?.index ?? 0;
    // if (string === 'ArrowLeft') {
    //   cursor -= 1;
    // } else if (string === 'ArrowRight') {
    //   cursor += 1;
    // } else if (string === 'ArrowUp') {
    //   let new_line = indicies[closest_line - 1];
    //   cursor = new_line?.index ?? 0;
    //   cursor = Number.isNaN(cursor) ? 0 : cursor;
    //   cursor += Math.min(offset, new_line?.length ?? 0);
    // } else if (string === 'ArrowDown') {
    //   let new_line = indicies[closest_line + 1];
    //   cursor = new_line?.index ?? code.length;
    //   cursor = Number.isNaN(cursor) ? code.length : cursor;
    //   cursor += Math.min(offset, new_line?.length ?? 0);
    //   if (closest_line + 1 > line_offset + max_lines - 2) {
    //     line_offset += 1;
    //   }
    // } else if (string === 'Home') {
    //   cursor = current_line.index;
    // } else if (string === 'End') {
    //   let new_line = indicies[closest_line + 1];
    //   cursor = new_line?.index - 1 ?? code.length;
    //   cursor = Number.isNaN(cursor) ? code.length : cursor;
    // } else if (string === 'Backspace') {
    //   let splice = code.split('');
    //   splice.splice(cursor - 1, 1);
    //   code = splice.join('');
    //   cursor -= 1;
    // }
    // cursor = Math.max(0, Math.min(cursor, code.length));
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
  <div class="row">
    <button on:click={() => move('ArrowUp')}>Up</button>
    <button on:click={() => move('ArrowDown')}>Down</button>
  </div>
  <div class="row">
    <button on:click={() => move('ArrowLeft')}>Left</button>
    <button on:click={() => write('Enter')}>Enter</button>
    <button on:click={() => move('ArrowRight')}>Right</button>
  </div>
  <div class="row">
    <button on:click={() => move('Backspace')}>Del</button>
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

  #device {
    width: min-content;
  }
</style>
