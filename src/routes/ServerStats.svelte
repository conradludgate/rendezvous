<script lang="ts">
  import { update_server_error_rate, view_server_health } from "rendezvous";
  import Chart from "./Chart.svelte";

  const { key, version } = $props();
  let error_rate = $state(0.1);

  $effect(() => {
    update_server_error_rate(key, error_rate);
  });
</script>

<div>
  {#key version}
    <span class="stats">
      Health: {view_server_health(key).toString().substring(0, 4)}<br />
      <input type="range" bind:value={error_rate} min="0" max="1" step="0.01" style="width: 6em;"/>
    </span>

    <Chart {key} />
  {/key}
</div>

<style>
  div {
    display: flex;
    flex-direction: row;
    font-size: 2rem;
    align-items: center;
  }

  .stats {
    margin-right: 1em;
    max-width: 8em;
    text-wrap-mode: nowrap;
  }
</style>
