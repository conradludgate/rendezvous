<script lang="ts">
  import * as d3 from "d3";
  import { Load, view_server_load } from "rendezvous";

  let { key } = $props();

  let values = $derived.by(() => view_server_load(key));

  const width = 300;
  const height = 100;

  const x = d3.scaleLinear([0, 100], [0, width]);
  const y = d3.scaleLinear([0, 100], [height, 0]);

  let totalLine = d3
    .line<Load>()
    .x((_d, index, _data) => x(index))
    .y((d) => y(d.total));

  let errorLine = d3
    .line<Load>()
    .x((_d, index, _data) => x(index))
    .y((d) => y(d.errors));

  let totalPath = $derived(totalLine(values));
  let errorPath = $derived(errorLine(values));
</script>

<svg {width} {height}>
  <path d={totalPath} fill="none" stroke="black" />
  <path d={errorPath} fill="none" stroke="red" />
</svg>
