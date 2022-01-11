use rand::{Rng};

// You will need a function to create a random list of N integers, duplicates allowed.

pub fn create(size: usize, range: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..size)
    .map(|_| {
        let idx: i32 = rng.gen_range(0..range);
        idx
    })
    .collect();

    numbers
}