use std::{sync::atomic::AtomicI32, thread, time::Duration};

static MAX_PRIORITY: AtomicI32 = AtomicI32::new(0);
static MED_PRIORITY: AtomicI32 = AtomicI32::new(0);
static LOW_PRIORITY: AtomicI32 = AtomicI32::new(0);

fn max_priority() {
    thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max);
    loop {
        MAX_PRIORITY.fetch_add(1, std::sync::atomic::Ordering::Release);
        std::thread::yield_now();
    }
}

fn med_priority() {
    loop {
        MED_PRIORITY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn low_priority() {
    thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Min);
    loop {
        LOW_PRIORITY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}
fn main() {
    thread::spawn(max_priority);
    thread::spawn(med_priority);
    thread::spawn(low_priority);
    thread::sleep(Duration::from_secs(5));
    println!(
        "MAX_PRIORITY: {} \n MED_PRIORITY: {} \n LOW_PRIORITY: {}",
        MAX_PRIORITY.load(std::sync::atomic::Ordering::Relaxed),
        MED_PRIORITY.load(std::sync::atomic::Ordering::Relaxed),
        LOW_PRIORITY.load(std::sync::atomic::Ordering::Relaxed)
    );
}
