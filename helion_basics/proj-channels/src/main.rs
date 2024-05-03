use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (sender, receiver) = mpsc::channel();

    // thread::spawn(move || {
    //     sender.send("Witaj z wątku!").unwrap();
    // });

    // let message = receiver.recv().unwrap();
    // println!("Otrzymałem: {}", message);

    let (sender, receiver) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..5 {
        let sender_clone = sender.clone();
        let handle = thread::spawn(move || {
            let message = format!("Wiadomość z wątku: {}", i);
            sender_clone.send(message).unwrap();
            thread::sleep(Duration::from_millis(500 * i as u64));
        });
        handles.push(handle);
    }

    drop(sender);

    for received in receiver {
        println!("Otrzymano: {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
