<script lang="ts">
  import { onMount } from 'svelte';
  import type { KeyboardEventHandler } from 'svelte/elements';
  import _ from 'underscore';

  type Mode = 'edit' | 'run';
  let mode: Mode = 'edit';
  let cursor = 0;
  let canvas: HTMLCanvasElement | null = null;

  let code = `'(fn\n  2 2 +\n  'a def\n)\n\ncall`;
  // let code = `1234567890123456`;
  // let code = `1\n2\n3\n4\n5\n6\n7`;

  let waiting = false;
  type Response = {
    stack: Array<string>;
    error: string;
  };
  let result: Response = {
    stack: [],
    error: ''
  };

  async function execute() {
    waiting = true;

    let res = await fetch('http://127.0.0.1:7777/execute', {
      method: 'POST',
      body: code
    });

    result = await res.json();
    waiting = false;
  }

  function change_mode() {
    if (mode === 'edit') {
      mode = 'run';
    } else if (mode === 'run') {
      mode = 'edit';
    }
  }

  function draw() {
    // let time = Math.round(Date.now() / 500);
    // let show_cursor = time % 2 === 0;
    // if (show_cursor) {
    //   overlay_buffer = ' '.repeat(cursor) + '█';
    // }

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

    let max = 16 * 7;

    let formatted_code = code
      .split('\n')
      .map((s) => `:${s}`)
      .join('\n');
    let lines = [];
    let line = '';
    for (let si in formatted_code.split('')) {
      let i = parseInt(si);
      let char = formatted_code[i];
      if ((line.length % 16 === 0 && line.length > 0) || char === '\n') {
        lines.push(line);
        line = '';
      }

      if (char !== '\n') {
        line += char;
      }
    }
    lines.push(line);

    const max_lines = 7;
    const max_chars = 16;

    let buffer_window = lines.slice(0, max_lines);

    debugger;

    for (let y = 0; y < max_lines; y++) {
      for (let x = 0; x < max_chars; x++) {
        if (buffer_window[y][x]) {
          c.fillText(buffer_window[y][x], x * 14.4, (y + 1) * 20);
        }
      }
    }

    // let missing = max - code.length;
    // let padded = code + ' '.repeat(missing);
    // let chunks = _.chunk(padded.split(''), 16);
    // for (let i = 0; i < 7; i++) {
    //   c.fillText(chunks[i].join(''), 0, (i + 1) * 20);
    // }
  }

  onMount(() => {
    setInterval(draw, 1000 / 10);
  });

  const keypress: KeyboardEventHandler<HTMLInputElement> = (e) => {
    e.preventDefault();

    if (e.key === 'Enter' && e.ctrlKey) {
      e.preventDefault();
      execute();
      return;
    }

    if (e.key === 'Enter') {
      cursor += 1;
      code += '\n';
    }

    if (e.key.length === 1) {
      cursor += 1;
      code += e.key;
    }
  };

  const keyup: KeyboardEventHandler<HTMLInputElement> = (e) => {
    if (e.key === 'Backspace') {
      e.preventDefault();
      let splice = code.split('');
      splice.splice(cursor - 1, 1);

      code = splice.join('');
      cursor -= 1;
    }
  };
</script>

{#if waiting}
  <pre>executing...</pre>
{:else if result.error}
  <pre>error: {result.error}</pre>
{:else}
  <pre>stack: {result.stack.join(', ')}</pre>
{/if}

<div id="device">
  <canvas id="buffer" bind:this={canvas} width="231px" height="143px"></canvas>
  <input type="text" on:keydown={keyup} on:keypress={keypress} />
  <button on:click={change_mode}>Mode</button>
</div>

<style lang="scss">
  #device {
    display: flex;
    flex-direction: column;
    width: min-content;
  }
</style>
