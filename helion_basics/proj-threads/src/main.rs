use std::thread;
use std::thread::sleep;

fn main() {
    let _handle = thread::spawn(|| {
        println!("Sleeping...");
        sleep(std::time::Duration::from_secs(1));

        println!("Witaj z nowego wątku");
    });

    println!("Witaj z wątku głównego");

    _handle.join().unwrap();

    let mut threads = vec![];
    
    for i in 0..10 {
        let _th = thread::spawn(move || {
            println!("Witaj w wątku nr: {}", i);
        });
        threads.push(_th);
    }

    for _th in threads {
        _th.join().unwrap();
    }
}
