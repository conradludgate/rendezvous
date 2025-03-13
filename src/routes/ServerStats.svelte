<script lang="ts">
  import { view_server_health } from "rendezvous";
  import Chart from "./Chart.svelte";

  let { key, error_rate, version } = $props();
</script>

<div>
  {#key version}
    <span class="stats">
      Health: {view_server_health(key)}<br />
      Error Rate: {error_rate * 100}%
    </span>

    <Chart {key}/>
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
  }
</style>
