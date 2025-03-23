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
    <Chart {key} />
  {/key}

  <div class="stats">
    {#key version}
      <span class="health">
        Health: {view_server_health(key).toString().substring(0, 4)}
      </span>
    {/key}

    <input
      class="error"
      type="range"
      bind:value={error_rate}
      min="0"
      max="1"
      step="0.01"
      style="width: 6em;"
    />
  </div>
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
    font-size: 2rem;
    align-items: center;
  }

  .stats {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
  }
</style>
