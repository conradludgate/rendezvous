<script lang="ts">
  import { update_aimd } from "rendezvous";

  let inc = $state(0.01);
  let dec = $state(0.95);

  $effect(() => {
    update_aimd(inc, dec);
  });
</script>

<div>
  INC <input type="number" bind:value={inc} style="width: 4em;"/>
  |
  DEC <input type="number" bind:value={dec} style="width: 4em;"/>
</div>

<style>
</style>
