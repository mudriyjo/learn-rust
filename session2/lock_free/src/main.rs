use std::{thread, time::Duration};

use dashmap::DashMap;
use once_cell::sync::Lazy;

static MAP: Lazy<DashMap<u32,u32>> = Lazy::new(DashMap::new);

fn main() {
    for i in 0..100 {
        thread::spawn(move || {
            loop {
                if let Some(mut value) = MAP.get_mut(&i) {
                    *value += 1;
                } else {
                    MAP.insert(i, i);
                }
            }
        });
    }

    thread::sleep(Duration::from_secs(1));
    println!("{MAP:?}");
}
