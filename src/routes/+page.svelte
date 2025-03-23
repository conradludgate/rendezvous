<script lang="ts">
  import Servers from "./Servers.svelte";
  import Aimd from "./Aimd.svelte";
  import Run from "./Run.svelte";

	import { onMount } from "svelte";
	import init from "rendezvous";
  import Zipf from "./Zipf.svelte";

  let initialised = $state(false);
	onMount(async () => {
    await init();
    initialised = true;
  });

  let version = $state(0);
</script>

<svelte:head>
  <title>Rendezvous Playground</title>
  <meta name="description" content="A loadbalancer demo that features rendezvous hashing and health info for routing preference" />
</svelte:head>

<section>
  {#if initialised}
    <Aimd />
    <Zipf />
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
