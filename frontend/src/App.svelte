<script lang="ts">
  import { onMount } from 'svelte';
  import __wbg_init from './pkg/backend';
  import WithWasm from './WithWasm.svelte';

  let wasm = false;
  onMount(async () => {
    let result = await __wbg_init();

    window.wasm = result;
    wasm = true;
  });
</script>

{#if wasm}
  <WithWasm />
{/if}
