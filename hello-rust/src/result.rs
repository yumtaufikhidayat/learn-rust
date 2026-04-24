use std::fs::File;
use std::io::prelude::*;

pub fn use_result() {
    let teks_angka = "42";
    let hasil_parse = teks_angka.parse::<i32>();  // Tipe hasil: Result<i32, ParseIntError>

    match hasil_parse {
        Ok(n) => println!("{} dikali 2 adalah {}.", n, n * 2),
        Err(e) => println!("Terjadi kesalahan parsing: {}", e),
    }

    let teks_invalid = "42abc";
    match teks_invalid.parse::<i32>() {
        Ok(n) => println!("Berhasil parse: {}", n),
        Err(e) => println!("Gagal parse '{}': {}", teks_invalid, e),
    }
}

pub fn use_custom_result() {
    let res1 = "100".parse::<i32>();      // Ok(100)
    let res2 = "abc".parse::<i32>();      // Err(ParseIntError)

    // is_ok dan is_err
    println!("res1.is_ok()? {}", res1.is_ok());   // true, karena 100 bisa di-parse
    println!("res2.is_err()? {}", res2.is_err()); // true, karena "abc" gagal di-parse

    // unwrap_or_else dengan closure
    let val1 = res1.unwrap_or_else(|_| -1);
    println!("res1.unwrap_or_else -> {}", val1);
    let val2 = res2.unwrap_or_else(|e| {
        println!("Terjadi error parsing: {}", e);
        -1  // nilai default jika error
    });
    println!("res2.unwrap_or_else -> {}", val2);
}

pub fn real_case_result() {
    let mut file = match File::open("nama.rs") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Gagal membuka file: {}", e);
            return;
        }
    };

    let mut isi = String::new();
    if let Err(e) = file.read_to_string(&mut isi) {
        eprintln!("Gagal membaca isi file: {}", e);
        return;
    }

    println!("Isi file:\n{}", isi);
}