fn jumlah_dua<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

pub fn use_generic() {
    let a = 2;
    let b = 3;

    println!("Hasil penjumlahan {} dan {} adalah: {}", a, b, jumlah_dua(a, b));
}