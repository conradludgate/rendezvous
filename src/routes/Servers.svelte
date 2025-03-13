<script lang="ts">
  import { add_server, remove_server, ServerKey } from "rendezvous";
  import { SvelteMap } from "svelte/reactivity";
  import ServerStats from "./ServerStats.svelte";

  let { version } = $props();

  const servers: SvelteMap<bigint, [ServerKey, number]> = new SvelteMap();

  $inspect(servers);
</script>

<div class="counter">
  {#each servers as [id, [key, error_rate]] (id)}
    <div class="server">
      <button
        onclick={() => {
          remove_server(key);
          servers.delete(id);
        }}
        aria-label="Delete a server"
      >
        <svg aria-hidden="true" viewBox="0 0 1 1">
          <path d="M0,0.5 L1,0.5" />
        </svg>
      </button>
      <ServerStats {key} {error_rate} {version} />
    </div>
  {/each}

  <div class="server">
    <button
      onclick={async () => {
        var key = add_server(0.1);
        console.log(key);
        servers.set(key.as_number(), [key, 0.1]);
      }}
      aria-label="Add a new server"
    >
      <svg aria-hidden="true" viewBox="0 0 1 1">
        <path d="M0,0.5 L1,0.5 M0.5,0 L0.5,1" />
      </svg>
    </button>

    <span> Add Server </span>
  </div>
</div>

<style>
  .counter {
    display: flex;
    flex-direction: column;
    row-gap: 1em;

    border-top: 1px solid rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
  }

  .counter button {
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

  .counter button:hover {
    background-color: var(--color-bg-1);
  }

  .counter > .server {
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
