# rendezvous-playground

I'm designing a load balancer at the moment. There's a few properties I care about
1. Automatic load reduction when a server is unhealthy
2. High cache hit rate
3. A/B testing functionality

I thought I could use some combination of rendezvous hashing + a self-reported health status
to get all 3.

This wasm-based webapp is a simulator of how it behaves with different configurations.

* Server health is determined using an AIMD, triggered by errors. AIMD values are configurable.
* Server errors are random based on the defined error rate slider.
* Servers have a LRU cache of 1000 items.
* Load is generated using a Zipf distribution of 100,000 items. The power-factor `s` is configurable.
* Servers can be added and removed on demand. The black line is total load, the red line is errors, the green line is cache hit-rate.
* Running the simulation can be done by hitting the "play/pause" button. Tune the RPM and steps accordingly to speed up or slow down as desired.
