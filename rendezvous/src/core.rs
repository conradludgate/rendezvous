use std::{
    collections::VecDeque,
    hash::{BuildHasher, Hash},
};

use foldhash::quality::FixedState;
use itertools::Itertools;
use rand::Rng;
use rand_distr::{Distribution, Zipf};
use slotmap::SlotMap;
use wasm_bindgen::prelude::*;

const K: usize = 3;
pub const LOAD_BUFFER: usize = 100;

#[derive(Debug)]
pub struct Server {
    pub seed: u64,
    pub health: f64,
    pub error_rate: f64,
    /// (total, errors)
    pub load: VecDeque<Load>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Load {
    pub total: u32,
    pub errors: u32,
}

impl Server {
    fn run(&mut self, aimd: Aimd, rng: &mut impl Rng) {
        let load = self.load.back_mut().unwrap();

        let error = rng.random_bool(self.error_rate);
        if error {
            self.health = aimd.dec(self.health);
            load.total += 1;
            load.errors += 1;
        } else {
            self.health = aimd.inc(self.health);
            load.total += 1;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Aimd {
    pub min: f64,
    pub max: f64,
    pub inc: f64,
    pub dec: f64,
}

slotmap::new_key_type! {
    #[wasm_bindgen]
    pub struct ServerKey;
}

impl Aimd {
    fn inc(&self, x: f64) -> f64 {
        (x + self.inc).clamp(self.min, self.max)
    }

    fn dec(&self, x: f64) -> f64 {
        (x * self.dec).clamp(self.min, self.max)
    }
}

pub fn run_load_inner(s: &mut SlotMap<ServerKey, Server>, aimd: Aimd, n: usize) {
    if s.is_empty() {
        return;
    }

    s.iter_mut().for_each(|(_, s)| {
        while s.load.len() >= LOAD_BUFFER {
            s.load.pop_front();
        }
        s.load.push_back(Load {
            total: 0,
            errors: 0,
        })
    });

    let mut rng = rand::rng();

    let zipf = Zipf::new(100000.0, 0.1).unwrap();
    for _ in 0..n {
        let t = zipf.sample(&mut rng) as u64;
        run_test(s, aimd, t, &mut rng);
    }
}

fn run_test<T: Hash>(s: &mut SlotMap<ServerKey, Server>, aimd: Aimd, t: T, rng: &mut impl Rng) {
    let distr = distr(s, K, &t);
    let choice = choose_from_distr(&distr, rng);
    s[*choice].run(aimd, rng);
}

fn choose_from_distr<'a, T>(distr: &'a [(T, f64)], rng: &mut impl Rng) -> &'a T {
    let x = rng.random_range(0.0..1.0);
    for i in distr {
        if x < i.1 {
            return &i.0;
        }
    }
    unreachable!()
}

fn distr<T: Hash>(servers: &SlotMap<ServerKey, Server>, k: usize, t: &T) -> Vec<(ServerKey, f64)> {
    let topk = servers
        .iter()
        .map(|(i, s)| {
            let hash = FixedState::with_seed(s.seed).hash_one(t);
            (i, hash)
        })
        .k_largest_by_key(k, |(_i, hash)| *hash)
        .map(|(i, _hash)| i);

    let mut scores = topk
        .enumerate()
        .map(|(idx, i)| {
            let order_score = 0.5 / (idx + 1) as f64;
            let score = order_score * servers[i].health;
            (i, score)
        })
        .collect::<Vec<_>>();

    // softmax each entry (as a PDF) and calculate the CDF in-place.
    let mut accum = 0.0;
    let sum = scores.iter().map(|(_, score)| *score).sum::<f64>();
    scores.iter_mut().for_each(|(_i, score)| {
        let p = *score / sum;

        *score = (accum + p).clamp(0.0, 1.0);
        accum = *score;
    });

    // to account for floating point errors
    scores.last_mut().unwrap().1 = 1.0;

    scores
}

#[test]
fn foo() {
    let s = |seed, health, error_rate| Server {
        seed,
        health,
        error_rate,
        load: VecDeque::new(),
    };

    let aimd = Aimd {
        min: 0.01,
        max: 1.0,
        inc: 0.01,
        dec: 0.9,
    };

    let mut servers = SlotMap::<ServerKey, Server>::with_key();
    servers.insert(s(0, 1.0, 0.05));
    servers.insert(s(1, 1.0, 0.05));
    servers.insert(s(2, 1.0, 0.05));
    servers.insert(s(3, 1.0, 0.05));
    servers.insert(s(4, 1.0, 0.05));
    servers.insert(s(5, 1.0, 0.05));
    servers.insert(s(6, 1.0, 0.05));
    servers.insert(s(7, 1.0, 0.05));
    servers.insert(s(8, 1.0, 0.05));
    servers.insert(s(9, 0.01, 0.25));

    run_load_inner(&mut servers, aimd, 10000);
    dbg!(&servers);
}
