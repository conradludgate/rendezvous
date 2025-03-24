use std::{
    collections::VecDeque,
    hash::{BuildHasher, Hash},
};

use foldhash::quality::FixedState;
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use rand::Rng;
use rand_distr::{Distribution, Uniform, Zipf};
use slotmap::SlotMap;
use wasm_bindgen::prelude::*;

const CACHE_SIZE: usize = 1000;
const K: usize = 3;
pub const LOAD_BUFFER: usize = 100;

#[derive(Debug)]
pub struct Server {
    // config
    pub seed: u64,
    pub error_rate: f64,

    // properties
    pub health: f64,
    pub load: VecDeque<Load>,
    pub cache: LinkedHashMap<u64, ()>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Load {
    pub total: u32,
    pub errors: u32,
    pub cached: u32,
}

impl Server {
    fn run(&mut self, aimd: Aimd, rng: &mut impl Rng, t: u64) {
        let load = self.load.back_mut().unwrap();

        let error = rng.random_bool(self.error_rate);
        if error {
            self.health = aimd.dec(self.health);
            load.total += 1;
            load.errors += 1;
        } else {
            self.health = aimd.inc(self.health);
            load.total += 1;

            if self.cache.get_refresh(&t).is_some() {
                load.cached += 1;
            } else {
                if self.cache.len() == CACHE_SIZE {
                    self.cache.pop_front();
                }
                self.cache.insert(t, ());
            }
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

pub fn run_load_inner(s: &mut SlotMap<ServerKey, Server>, aimd: Aimd, n: usize, zipf_s: f64) {
    if s.is_empty() {
        return;
    }

    s.iter_mut().for_each(|(_, s)| {
        s.load.pop_front();
        s.load.push_back(Load {
            total: 0,
            errors: 0,
            cached: 0,
        })
    });

    let mut rng = rand::rng();

    let zipf = Zipf::new(100000.0, zipf_s).unwrap();
    for _ in 0..n {
        let input = zipf.sample(&mut rng) as u64;
        let server = select(s, input, &mut rng);
        server.run(aimd, &mut rng, input);
    }
}

fn select<'a>(s: &'a mut SlotMap<ServerKey, Server>, t: u64, rng: &mut impl Rng) -> &'a mut Server {
    let distr = distr(s, K, &t);
    let choice = distr.sample(rng);
    &mut s[choice]
}

fn distr<T: Hash>(servers: &SlotMap<ServerKey, Server>, k: usize, t: &T) -> Distr<ServerKey> {
    let weights = rendezvous_hash(servers, k, t)
        .enumerate()
        .map(|(idx, i)| {
            // combine the order placement
            // with the health value
            let order_score = 0.5 / (idx + 1) as f64;
            let score = order_score * servers[i].health;
            (i, score)
        })
        .collect::<Vec<_>>();

    get_cdf(weights)
}

fn rendezvous_hash<T: Hash>(
    servers: &SlotMap<ServerKey, Server>,
    k: usize,
    t: &T,
) -> impl Iterator<Item = ServerKey> {
    servers
        .iter()
        // hash the key with each server seed
        .map(|(i, s)| (i, FixedState::with_seed(s.seed).hash_one(t)))
        // get the servers with the k largest hashes
        .k_largest_relaxed_by_key(k, |(_i, hash)| *hash)
        // discard the hash
        .map(|(i, _hash)| i)
}

fn get_cdf<T>(mut weights: Vec<(T, f64)>) -> Distr<T> {
    // calculate the CDF in-place.
    let mut accum = 0.0;
    weights.iter_mut().for_each(|(_i, p)| {
        *p += accum;
        accum = *p;
    });

    let last = weights.last().unwrap().1;
    Distr {
        sampler: Uniform::new(0.0, last).unwrap(),
        distr: weights,
    }
}

struct Distr<T> {
    sampler: Uniform<f64>,
    distr: Vec<(T, f64)>,
}

impl<T: Clone> Distribution<T> for Distr<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        let sample = self.sampler.sample(rng);
        let i = self.distr.partition_point(|(_, weight)| weight <= &sample);
        self.distr[i].0.clone()
    }
}
