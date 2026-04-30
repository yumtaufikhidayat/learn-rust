macro_rules! salam {
    ($nama:expr) => {
        println!("Halo, {}!", $nama);
    };
}

pub fn use_macro() {
    salam!("Dicoding");  // akan ekspansi menjadi println!("Halo, Dicoding!");
}