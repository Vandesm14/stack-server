<script lang="ts">
	import type { KeyboardEventHandler } from 'svelte/elements';

  type Response = {
    stack: Array<string>,
    error: string,
  }

  let code = '';
  let result: Response = {
    stack: [],
    error: ''
  };

  async function execute() {
    let res = await fetch("http://127.0.0.1:7777/execute", {
      method: 'POST',
      body: code,
    });

    result = await res.json();
  }

  const keydown: KeyboardEventHandler<HTMLTextAreaElement> = (e) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      execute();
    }
  }
</script>

{#if result.error}
<pre>error: {result.error}</pre>
{:else}
<pre>{result.stack.join(', ')}</pre>
{/if}
<textarea bind:value={code} on:keydown={keydown}></textarea>
<button on:click={execute}>Execute</button>

<style>
  textarea {
    height: 200px;
    width: 300px;
  }
</style>
