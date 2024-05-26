<script lang="ts">
	import type { KeyboardEventHandler } from 'svelte/elements';

  type Response = {
    stack: Array<string>,
    error: string,
  }

  let waiting = false;
  let code = '';
  let result: Response = {
    stack: [],
    error: ''
  };

  async function execute() {
    waiting = true;

    let res = await fetch("http://127.0.0.1:7777/execute", {
      method: 'POST',
      body: code,
    });

    result = await res.json();
    waiting = false;
  }

  const keydown: KeyboardEventHandler<HTMLTextAreaElement> = (e) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      execute();
    }
  }
</script>

{#if waiting}
  <pre>executing...</pre>
{:else}
  {#if result.error}
  <pre>error: {result.error}</pre>
  {:else}
  <pre>stack: {result.stack.join(', ')}</pre>
  {/if}
{/if}

<textarea bind:value={code} on:keydown={keydown}></textarea>
<button on:click={execute}>Execute</button>

<style>
  textarea {
    height: 200px;
    width: 300px;
  }
</style>
