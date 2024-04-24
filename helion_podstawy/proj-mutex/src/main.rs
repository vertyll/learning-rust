// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}", m);
// }

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct History {
    log: Vec<String>,
}

impl History {
    fn new() -> History {
        History { log: Vec::new() }
    }

    fn add_record(&mut self, record: String) {
        self.log.push(record);
    }
}

fn main() {
    let history = Arc::new(Mutex::new(History::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let history_clone = Arc::clone(&history);
        let handle = thread::spawn(move || {
            let mut history = history_clone.lock().unwrap();
            history.add_record(format!("Wątek {} wywołany", i));
            thread::sleep(Duration::from_millis(10 * i as u64));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let history = history.lock().unwrap();
    for record in history.log.iter() {
        println!("{}", record);
    }
}