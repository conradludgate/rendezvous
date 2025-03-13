<script lang="ts">
  import { onMount } from "svelte";

  import Servers from "./Servers.svelte";

  import init, { run_load } from "rendezvous";

  let version = $state(0);

  onMount(init);
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <button
    onclick={() => {
      run_load(100);
      version = version + 1;
    }}
  >
    Run
  </button>

  <Servers {version} />
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex: 0.6;
  }
</style>
