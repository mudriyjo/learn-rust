use std::{sync::Mutex, thread, time::Duration};

fn op1(state: &Mutex<u32>) {
    let lock = state.lock();
    op2(state);
}

fn op2(state: &Mutex<u32>) {
    let lock = state.lock();
    op1(state);
}

fn dead_lock() {
    let mutex = Mutex::new(0);
    op1(&mutex);
    op2(&mutex);
}

fn op1_v2(state: &Mutex<u32>) {
    let mut attempt = 5;
    thread::sleep(Duration::from_secs(1));
    loop {
        let lock = state.try_lock();
        match lock {
            Ok(res) => {
                println!("Get res op1: {}", res);
                op2_v2(state);
            }
            Err(_) => {
                if attempt <= 0 {
                    break;
                } else {
                    println!("attempt: {attempt}");
                    attempt -= 1;
                }
            }
        }
    }
    println!("Op1 Finished...")
}

fn op2_v2(state: &Mutex<u32>) {
    let mut attempt = 5;
    loop {
        thread::sleep(Duration::from_secs(1));
        let lock = state.try_lock();
        match lock {
            Ok(res) => {
                println!("Get res op2: {}", res);
                op1_v2(state);
            }
            Err(_) => {
                if attempt <= 0 {
                    break;
                } else {
                    println!("attempt: {attempt}");
                    attempt -= 1;
                }
            }
        }
    }
}
fn try_dead_lock() {
    let mutex = Mutex::new(0);
    op1_v2(&mutex);
    op2_v2(&mutex);
}

static MUTEX: Mutex<u32> = Mutex::new(0);

fn poisoning() {
    let lock = MUTEX.lock().unwrap();
    lock.checked_add(1);
    panic!("Panic....")
}

fn main() {
    // dead_lock();
    // try_dead_lock();
    let mut mutex = Mutex::new(0);
    {
        thread::spawn(move || poisoning()).join();
    }
    let lock = MUTEX.lock();
    match lock {
        Ok(_lock) => println!("get lock"),
        Err(err) => {
            println!("Mutex poisoned");
            let res = err.into_inner();
            println!("Restore Mutex value: {res}");
        }
    }
    println!("Restored value: {}", mutex.lock().unwrap());
}
