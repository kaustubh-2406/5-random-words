<script lang="ts">
  import init, { get_n_words } from 'rnd-word';
  import { onMount } from 'svelte';

  let words = [];
  let ready = false;
  
  function handleClick() {
    if (ready) {
      words = get_n_words(5).split(':');
      console.log(words)
    }
  }

  onMount(async () => {
    await init()
    ready = true;
  })
</script>

<main>
  <div class="mb-4">
    <h1>5 Random Words</h1>
    <div>Get five random words for projects some fun project ideas...</div>
  </div>

  <!-- Show loading untill wasm module is loaded.. -->
  {#if !ready}
    <div>Loading....</div>
  {:else}
    <button on:click={handleClick}>Get random words</button>

    <!-- If words are generated, then show heading -->
    {#if words.length != 0}
      <h4>Generated words</h4>
    {/if}

    <!-- Container div -->
    <div class="d-flex">
      {#each words as word}
        <div class="light">{word}</div>
      {/each}
    </div>
  {/if}
</main>

<style>
  .mb-4 {
    margin-bottom: 2rem;
  }

  .light {
    color: black;
    padding: 5px 10px;
    border-radius: 20px;
    background-color: rgba(255, 255, 255, 0.5);
  }

  .d-flex {
    display: flex;
    gap: 1rem;
    justify-content: center;
    align-items: center;
  }
</style>
