use std::thread;

fn main() {
    let ids = core_affinity::get_core_ids().unwrap();
    let handles = ids.into_iter().map(|id| {
        thread::spawn(move || {
            let success = core_affinity::set_for_current(id);
            if success {
                println!("Thread affinity setuped for id: {id:?}");
            } else {
                println!("Can't setup thread affinity for id: {id:?}")
            }
        })
    }).collect::<Vec<_>>();

    handles.into_iter().for_each(|h| {
        h.join().unwrap();
    })
}
