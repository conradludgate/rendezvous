<script lang="ts">
  import * as d3 from "d3";
  import { Load, view_server_load } from "rendezvous";

  let { key } = $props();

  let values = $derived.by(() => view_server_load(key));
  let errors = $derived(values[values.length - 1].errors);

  const width = 500;
  const height = 200;

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

  const percentY = d3.scaleLinear([0, 1], [height, 0]);
  let cachedLine = d3
    .line<Load>()
    .x((_d, index, _data) => x(index))
    .y((d) => percentY(d.total === 0 ? 0 : d.cached / d.total));

  let totalPath = $derived(totalLine(values));
  let errorPath = $derived(errorLine(values));
  let cachedPath = $derived(cachedLine(values));
</script>

<div class="data">
  <span class="errors">{errors}</span>
  <svg {width} {height}>
    <path
      d="
        M 0 0
        l {0} {height}
        l {width} 0
        l {0} {-height}
        l {-width} 0

        M {width} {(height * 1) / 5}
        l {-5} {0}

        M {width} {(height * 2) / 5}
        l {-5} {0}

        M {width} {(height * 3) / 5}
        l {-5} {0}

        M {width} {(height * 4) / 5}
        l {-5} {0}
      "
      fill="none"
      stroke="black"
    />
    <path d={totalPath} fill="none" stroke="black" />
    <path d={errorPath} fill="none" stroke="red" />
    <path d={cachedPath} fill="none" stroke="green" />
  </svg>
</div>

<style>
  .data {
    position: relative;
  }
  .data .errors {
    position: absolute;
    right: 0.5em;
    top: 0;
    color: red;
  }
</style>
