<script lang="ts">
  import { add_server, remove_server, ServerKey } from "rendezvous";
  import { SvelteSet } from "svelte/reactivity";
  import Server from "./Server.svelte";

  let { version } = $props();

  const servers: SvelteSet<ServerKey> = new SvelteSet();
</script>

<div class="servers">
  {#each servers as key (key)}
    <Server
      {key}
      {version}
      onremove={() => {
        remove_server(key);
        servers.delete(key);
      }}
    />
  {/each}

  <!-- <div class="add_server"> -->
  <button
    class="add_server"
    onclick={async () => {
      var key = add_server(0.1);
      console.log(key);
      servers.add(key);
    }}
    aria-label="Add a new server"
  >
    <svg aria-hidden="true" viewBox="0 0 1 1">
      <path d="M0,0.5 L1,0.5 M0.5,0 L0.5,1" />
    </svg>
  </button>

  <!-- <span> Add Server </span> -->
  <!-- </div> -->
</div>

<style>
  .servers {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    row-gap: 2em;
    column-gap: 2em;
    align-items: center;
    justify-content: space-evenly;

    border-top: 1px solid rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
  }

  .add_server:hover {
    background-color: var(--color-bg-1);
  }

  .add_server {
    width: 500px;
    height: calc(200px);
    margin-bottom: 1.5em;

    display: flex;
    flex-direction: row;
    font-size: 2rem;

    align-items: center;
    justify-content: center;
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
