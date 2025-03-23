<script lang="ts">
  import { run_load } from "rendezvous";

  let steps = $state(100);
  let rpm = $state(60);
  let playing = $state(false);

  let { version = $bindable(), ...props } = $props();

  $effect(() => {
    const interval = setInterval(() => {
      if (playing) {
        run_load(steps);
        version = version + 1;
      }
    }, 60000 / rpm);

    return () => {
      clearInterval(interval);
    };
  });
</script>

<div>
  <button onclick={() => playing = !playing}>
    {#if playing}
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-play-fill" viewBox="0 0 16 16">
      <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393"/>
    </svg>
    {:else}
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pause-fill" viewBox="0 0 16 16">
      <path d="M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5m5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5"/>
    </svg>
    {/if}
  </button>
  |
  <input type="number" bind:value={rpm} step={rpm * 0.1} style="width: 4em;"/> RPM
  |
  <input type="number" bind:value={steps} step={steps * 0.1} style="width: 4em;"/> STEPS
</div>


<style>
</style>
