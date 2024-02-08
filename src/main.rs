use std::{sync::atomic::AtomicUsize, thread, time::Duration};
use std::sync::atomic::Ordering;


fn main() {
    let num_done =  AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..30 {
                println!("{}",i);
                thread::sleep(Duration::from_secs(1));
                num_done.store(i + 1, Ordering::Relaxed);
            }
        });

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 30 {break;}
            println!("Working.. {n}/30 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}
