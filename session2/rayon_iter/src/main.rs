use std::time::Instant;

use rayon::prelude::*;

fn sequential_is_prime(n: u32) -> bool {
    let range: Vec<u32> = (0..n / 2).collect();
    range.into_iter().all(|num| num % n != 0)
}

fn sequential_iteration() -> Vec<u32> {
    let numbers: Vec<u32> = (0..10_000).collect();
    let mut result: Vec<u32> = numbers
        .into_iter()
        .filter(|num| sequential_is_prime(*num))
        .collect();
    result.sort_unstable();
    result
}

fn parallel_is_prime(n: u32) -> bool {
    let range: Vec<u32> = (0..n / 2).collect();
    range.into_par_iter().all(|num| num % n != 0)
}

fn parallel_iteration() -> Vec<u32> {
    let numbers: Vec<u32> = (0..10_000).collect();
    let mut result: Vec<u32> = numbers
        .into_par_iter()
        .filter(|num| parallel_is_prime(*num))
        .collect();
    result.par_sort_unstable();
    result
}

fn main() {
    let now = Instant::now();
    let result = sequential_iteration();
    let after = Instant::now();
    println!(
        "There are {} prime numbers callculated sequintially in {} seconds.",
        result.len(),
        after.checked_duration_since(now).unwrap().as_secs_f32()
    );

    let now = Instant::now();
    let result = parallel_iteration();
    let after = Instant::now();
    println!(
        "There are {} prime numbers callculated parallel in {} seconds.",
        result.len(),
        after.checked_duration_since(now).unwrap().as_secs_f32()
    );
}
