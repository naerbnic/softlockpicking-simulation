use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use rayon::prelude::*;

struct RollSimulator<'a, T> {
    rng: &'a mut T,
    numbers: [u32; 4],
}

impl<'a, T> RollSimulator<'a, T>
where
    T: rand::Rng,
{
    pub fn new(rng: &'a mut T) -> Self {
        RollSimulator {
            rng,
            numbers: [0; 4],
        }
    }

    pub fn roll_one(&mut self) {
        let roll = rand::Rng::gen_range(self.rng, 0..4);
        self.numbers[roll] += 1;
    }

    pub fn roll_n(&mut self, n: u32) {
        for _ in 0..n {
            self.roll_one();
        }
    }

    pub fn curr_rolls(&self) -> &[u32] {
        &self.numbers
    }
}

fn main() {
    const NUM_SIMULATIONS: u32 = 1_000_000_000;
    let max_ones = AtomicU32::new(0);
    let count = AtomicU64::new(0);
    let start_time = std::time::Instant::now();
    let message_frequency = std::time::Duration::from_secs(10);
    let next_message_time = std::sync::Mutex::new(start_time + message_frequency);
    (0..NUM_SIMULATIONS).into_par_iter().for_each(|_| {
        let mut rng = rand::rngs::ThreadRng::default();
        let mut sim = RollSimulator::new(&mut rng);
        sim.roll_n(231);
        let rolls = sim.curr_rolls();
        max_ones
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |curr_max| {
                Some(curr_max.max(rolls[0]))
            })
            .unwrap();
        let prev_value = count.fetch_add(1, Ordering::Relaxed);
        let curr_time = std::time::Instant::now();
        if prev_value % 100_000 == 0 {
            let mut next_message_time = next_message_time.lock().unwrap();
            if curr_time >= *next_message_time {
                println!(
                    "Iteration #{}, max ones: {}, elapsed_time: {:?}",
                    count.load(Ordering::Relaxed),
                    max_ones.load(Ordering::Relaxed),
                    curr_time.duration_since(start_time),
                );
                *next_message_time += message_frequency;
            }
        }
    });
    println!("Finished. Max ones: {}", max_ones.load(Ordering::Relaxed));
    println!("Duration: {:?}", start_time.elapsed());
}
