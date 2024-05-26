<script lang="ts">
  let code = '';

  let result = '';
  async function execute() {
    let res = await fetch("http://127.0.0.1:7777/execute", {
      method: 'POST',
      body: code,
    });

    result = await res.text()
  }
</script>

<pre>{result}</pre>
<textarea bind:value={code}></textarea>
<button on:click={execute}>Execute</button>