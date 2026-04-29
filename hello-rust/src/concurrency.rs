use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub fn use_concurrency() {
    thread::spawn(|| {
        println!("Halo dari thread lain!");
    });
    println!("Halo dari thread utama!");
}

pub fn use_concurrency_mpsc_channel() {
    // Membuat sebuah channel
    let (tx, rx) = mpsc::channel();

    // Membuat thread baru yang mengirim pesan melalui tx
    thread::spawn(move || {
        let message = "Hello dari thread baru!";
        tx.send(message).unwrap();
    });

    // Menerima pesan di thread utama
    let received = rx.recv().unwrap();
    println!("Pesan diterima: {}", received);
}

pub fn use_concurrency_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_cloned = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_cloned.lock().unwrap();
            *num += 1;  // mutasi nilai di dalam Mutex
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Hasil akhir: {}", *counter.lock().unwrap());
}