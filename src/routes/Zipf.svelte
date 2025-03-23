<script lang="ts">
  import { update_zipf_s } from "rendezvous";
  let s = $state(0.9);

  $effect(() => {
    update_zipf_s(s);
  });
</script>

<div>
  Zipf s: <input type="number" bind:value={s} style="width: 4em;"/>
</div>

<style>
</style>
