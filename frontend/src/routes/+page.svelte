<script lang="ts">
  import { onMount } from 'svelte';
  import type { KeyboardEventHandler } from 'svelte/elements';

  type Mode = 'edit' | 'run';
  let mode: Mode = 'edit';
  let buffer = '';
  let cursor = 0;
  let canvas: HTMLCanvasElement | null = null;

  let code = '';

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
    // } else {
    //   overlay_buffer = '';
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
    c.fillText('1234567890123456', 0, 20);
    c.fillText('1234567890123456', 0, 40);
    c.fillText('1234567890123456', 0, 60);
    c.fillText('1234567890123456', 0, 80);
    c.fillText('1234567890123456', 0, 100);
    c.fillText('1234567890123456', 0, 120);
    c.fillText('1234567890123456', 0, 140);
  }

  onMount(() => {
    setInterval(draw, 1000 / 20);
  });

  const keydown: KeyboardEventHandler<HTMLInputElement> = (e) => {
    e.preventDefault();

    if (e.key === 'Enter' && e.ctrlKey) {
      execute();
    }
  };

  const keypress: KeyboardEventHandler<HTMLInputElement> = (e) => {
    e.preventDefault();

    if (e.key) {
      cursor += 1;
      code += e.key;
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
  <input type="text" on:keydown={keydown} on:keypress={keypress} />
  <button on:click={change_mode}>Mode</button>
</div>

<style lang="scss">
  #device {
    display: flex;
    flex-direction: column;
    width: min-content;

    & > * {
      width: fit-content;
    }

    #buffer {
      // width: 16ch * 1.2;
      // height: 7ch * 2.25;
      background: black;
    }
  }
</style>
