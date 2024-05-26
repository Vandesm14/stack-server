<script lang="ts">
  import { onMount } from 'svelte';
  import type { KeyboardEventHandler } from 'svelte/elements';
  import _ from 'underscore';

  type Mode = 'edit' | 'run';
  let mode: Mode = 'edit';
  let cursor = 0;
  let canvas: HTMLCanvasElement | null = null;

  let code = `'(fn\n  2 2 +\n  'a def\n)\n\ncall`;
  $: indicies = newLineIndicies(code);
  // let code = `1234567890123456`;
  // let code = `1\n2\n3\n4\n5\n6\n7`;

  function newLineIndicies(code: string): Array<{ index: number; length: number }> {
    let lines = code.split('\n');
    let indicies: Array<{ index: number; length: number }> = [];
    let currentIndex = 0;
    for (let i = 0; i < lines.length; i++) {
      indicies.push({ index: currentIndex, length: lines[i].length });
      currentIndex += lines[i].length + 1; // +1 for the newline character
    }
    return indicies;
  }

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

    function generateLinesLookup(
      formatted_code: string,
      max_chars: number
    ): Array<[number, number]> {
      let lines_lookup: Array<[number, number]> = [];
      let x = 1; // Start from 1 to account for the ':' character
      let y = 0;

      for (let i = 0; i < formatted_code.length; i++) {
        if (formatted_code[i] === '\n') {
          // When a newline is encountered, increment y and reset x
          y += 1;
          x = 1; // Reset to 1 to account for the ':' character in the next line
        } else {
          lines_lookup.push([x, y]);
          x += 1;
          if (x > max_chars) {
            // If x exceeds max_chars, increment y and reset x
            y += 1;
            x = 1; // Reset to 1 to account for the ':' character in the next line
          }
        }
      }
      return lines_lookup;
    }
    let lines_lookup: Array<[number, number]> = generateLinesLookup(formatted_code, max_chars);

    for (let y = 0; y < max_lines; y++) {
      for (let x = 0; x < max_chars; x++) {
        if (lines_lookup.findIndex(([lx, ly]) => lx === x && ly === y) === cursor) {
          c.fillStyle = 'white';
          c.fillRect(x * 14.4, y * 20 + 2, 14.4, 22);
          c.fillStyle = 'black';
        }

        if (buffer_window[y]?.[x]) {
          c.fillText(buffer_window[y][x], x * 14.4, (y + 1) * 20);
        }

        c.fillStyle = 'white';
      }
    }
  }

  onMount(() => {
    setInterval(draw, 1000 / 30);

    const keydown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft') {
        cursor -= 1;
      } else if (e.key === 'ArrowRight') {
        cursor += 1;
      } else if (e.key === 'ArrowUp') {
        let closest_line = 0;
        for (let i in indicies) {
          if (indicies[i].index - 1 < cursor) {
            closest_line = parseInt(i);
          } else break;
        }

        let current_line = indicies[closest_line];
        let offset = cursor - current_line?.index ?? 0;

        let new_line = indicies[closest_line - 1];
        cursor = new_line?.index ?? 0;
        cursor = Number.isNaN(cursor) ? 0 : cursor;
        cursor += Math.min(offset, new_line?.length ?? 0);
      } else if (e.key === 'ArrowDown') {
        let closest_line = 0;
        for (let i in indicies) {
          if (indicies[i].index - 1 < cursor) {
            closest_line = parseInt(i);
          } else break;
        }

        let current_line = indicies[closest_line];
        let offset = cursor - current_line?.index ?? 0;

        let new_line = indicies[closest_line + 1];
        cursor = new_line?.index ?? code.length;
        cursor = Number.isNaN(cursor) ? code.length : cursor;
        cursor += Math.min(offset, new_line?.length ?? 0);
      } else if (e.key === 'Backspace') {
        e.preventDefault();
        let splice = code.split('');
        splice.splice(cursor - 1, 1);

        code = splice.join('');
        cursor -= 1;
      }

      cursor = Math.max(0, Math.min(cursor, code.length));
    };

    const keypress = (e: KeyboardEvent) => {
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
        code = code.slice(0, cursor) + e.key + code.slice(cursor);
        cursor += 1;
      }
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

<p>{cursor}</p>

<div id="device">
  <canvas id="buffer" bind:this={canvas} width="231px" height="143px"></canvas>
  <button on:click={change_mode}>Mode</button>
</div>

<style lang="scss">
  #device {
    display: flex;
    flex-direction: column;
    width: min-content;
  }
</style>
