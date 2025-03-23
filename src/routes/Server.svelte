<script lang="ts">
  import ServerStats from "./ServerStats.svelte";

  let { key, version, onremove } = $props();
</script>

<div class="server">
  <button onclick={onremove} aria-label="Delete a server">
    <svg aria-hidden="true" viewBox="0 0 1 1">
      <path d="M0,0.5 L1,0.5" />
    </svg>
  </button>
  <ServerStats {key} {version} />
</div>

<style>
  button {
    position: absolute;
    top: 0;
    left: 0;

    height: 2em;
    width: 2em;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 0;
    background-color: transparent;
    touch-action: manipulation;
    font-size: 2rem;
  }

  button:hover {
    background-color: var(--color-bg-1);
  }

  .server {
    position: relative;
    display: flex;
    flex-direction: row;
    font-size: 2rem;
    align-items: center;
  }

  svg {
    width: 25%;
    height: 25%;
  }

  path {
    vector-effect: non-scaling-stroke;
    stroke-width: 2px;
    stroke: #444;
  }
</style>
