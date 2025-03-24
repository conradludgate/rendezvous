use core::{Aimd, LOAD_BUFFER, Load, Server, ServerKey, run_load_inner};
use std::cell::{Cell, RefCell};

use linked_hash_map::LinkedHashMap;
use slotmap::SlotMap;
use wasm_bindgen::prelude::*;

mod core;

#[wasm_bindgen(start)]
pub fn main() {}

const START: f64 = 0.01;

thread_local! {
    static SERVERS: RefCell<SlotMap<ServerKey, Server>> = RefCell::new(SlotMap::with_key());

    static AIMD: Cell<Aimd> = const { Cell::new(Aimd{
        min: 0.01,
        max: 1.0,
        inc: 0.01,
        dec: 0.99,
    })};

    static S: Cell<f64> = const { Cell::new(0.9) };
}

#[wasm_bindgen]
pub fn add_server(error_rate: f64) -> ServerKey {
    SERVERS.with_borrow_mut(|s| {
        s.insert(Server {
            seed: rand::random(),
            health: START,
            error_rate,
            load: std::iter::repeat_n(
                Load {
                    total: 0,
                    errors: 0,
                    cached: 0,
                },
                LOAD_BUFFER,
            )
            .collect(),
            cache: LinkedHashMap::new(),
        })
    })
}

#[wasm_bindgen]
pub fn remove_server(i: ServerKey) {
    SERVERS.with_borrow_mut(|s| {
        s.remove(i);
    })
}

#[wasm_bindgen]
pub fn view_server_load(i: &ServerKey) -> Vec<Load> {
    SERVERS.with_borrow_mut(|s| s[*i].load.iter().copied().collect())
}

#[wasm_bindgen]
pub fn view_server_health(i: &ServerKey) -> f64 {
    SERVERS.with_borrow_mut(|s| s[*i].health)
}

#[wasm_bindgen]
pub fn update_server_error_rate(i: &ServerKey, error_rate: f64) {
    SERVERS.with_borrow_mut(|s| s[*i].error_rate = error_rate)
}

#[wasm_bindgen]
pub fn update_aimd(inc: f64, dec: f64) {
    AIMD.set(Aimd {
        inc,
        dec,
        min: 0.01,
        max: 1.0,
    });
}

#[wasm_bindgen]
pub fn update_zipf_s(s: f64) {
    S.set(s);
}

#[wasm_bindgen]
pub fn run_load(n: usize) {
    SERVERS.with_borrow_mut(|s| {
        run_load_inner(s, AIMD.get(), n, S.get());
    });
}
