<script lang="ts">
  import { onMount } from 'svelte';
  import _ from 'underscore';

  type Mode = 'edit' | 'run';
  let mode: Mode = 'edit';
  let cursor = 0;
  let line_offset = 0;
  let canvas: HTMLCanvasElement | null = null;

  let code = `'(fn!\n  2 2 +\n  'a def\n  a\n)\n\ncall`;
  $: indicies = newLineIndicies(code);

  function newLineIndicies(code: string): Array<{ index: number; end: number; length: number }> {
    let lines = code.split('\n');
    let indicies: Array<{ index: number; end: number; length: number }> = [];
    let currentIndex = 0;
    for (let i = 0; i < lines.length; i++) {
      indicies.push({
        index: currentIndex,
        end: currentIndex + lines[i].length,
        length: lines[i].length
      });
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

  const max_lines = 7;
  const max_chars = 16;

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

    lines = lines.slice(line_offset, max_lines + line_offset);
    lines = ['PROGRAM', ...lines];

    let cursor_pos: [number, number] = [0, 0];
    let counter = 0;
    for (let line in lines.slice(1)) {
      for (let char in lines[parseInt(line) + 1].split('')) {
        if (counter === cursor) {
          cursor_pos = [parseInt(char), parseInt(line)];
        }

        counter += 1;
      }
    }
    cursor_pos = [cursor_pos[0] + 1, cursor_pos[1] + 1];

    let chars = 0;
    for (let y = 0; y < max_lines; y++) {
      for (let x = 0; x < max_chars; x++) {
        let char = lines[y]?.[x];
        if (cursor_pos[0] === x && cursor_pos[1] === y) {
          c.fillStyle = 'white';
          c.fillRect(x * 14.4, y * 20 + 2, 14.4, 22);
          c.fillStyle = 'black';
        }

        if (char) {
          c.fillText(lines[y][x], x * 14.4, (y + 1) * 20);
          chars += 1;
        }

        c.fillStyle = 'white';
      }
    }
  })();

  onMount(() => {
    const keydown = (e: KeyboardEvent) => {
      let closest_line = 0;
      for (let i in indicies) {
        if (indicies[i].index <= cursor) {
          closest_line = parseInt(i);
        } else break;
      }

      let current_line = indicies[closest_line];
      let offset = cursor - current_line?.index ?? 0;

      if (e.key === 'ArrowLeft') {
        cursor -= 1;
      } else if (e.key === 'ArrowRight') {
        cursor += 1;
      } else if (e.key === 'ArrowUp') {
        let new_line = indicies[closest_line - 1];
        cursor = new_line?.index ?? 0;
        cursor = Number.isNaN(cursor) ? 0 : cursor;
        cursor += Math.min(offset, new_line?.length ?? 0);
      } else if (e.key === 'ArrowDown') {
        let new_line = indicies[closest_line + 1];
        cursor = new_line?.index ?? code.length;
        cursor = Number.isNaN(cursor) ? code.length : cursor;
        cursor += Math.min(offset, new_line?.length ?? 0);

        if (closest_line + 1 > line_offset + max_lines - 2) {
          line_offset += 1;
        }
      } else if (e.key === 'Home') {
        cursor = current_line.index;
      } else if (e.key === 'End') {
        let new_line = indicies[closest_line + 1];
        cursor = new_line?.index - 1 ?? code.length;
        cursor = Number.isNaN(cursor) ? code.length : cursor;
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
        code = code.slice(0, cursor) + '\n' + code.slice(cursor);
        cursor += 1;
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
