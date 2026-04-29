use std::thread;

pub fn use_concurrency() {
    thread::spawn(|| {
        println!("Halo dari thread lain!");
    });
    println!("Halo dari thread utama!");
}