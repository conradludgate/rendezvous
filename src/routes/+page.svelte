<script lang="ts">
  import Servers from "./Servers.svelte";
  import Aimd from "./Aimd.svelte";
  import Run from "./Run.svelte";

	import { onMount } from "svelte";
	import init from "rendezvous";

  let initialised = $state(false);
	onMount(async () => {
    await init();
    initialised = true;
  });

  let version = $state(0);
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  {#if initialised}
    <Aimd />
    <Run bind:version />
    <Servers {version} />
  {/if}
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
