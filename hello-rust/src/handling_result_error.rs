/*
    Tugas:
    Lengkapi fungsi-fungsi berikut untuk:
    - Membaca angka dari string menggunakan parsing dan mengembalikan Result
    - Menjumlahkan dua string yang merepresentasikan angka (dalam Result)
    - Menampilkan hasil jika parsing sukses, atau pesan error jika gagal
    - Menangani kasus None saat mengambil elemen dari vector

    Contoh Output:
    Sum is: 30
    Failed to parse input.
    Found: Some(99)
    No value at index
*/

fn parse_number(s: &str) -> Result<i32, String> {
    // TODO: Gunakan match untuk menangani hasil parsing string ke i32
    // Jika berhasil, kembalikan Ok(nilai)
    // Jika gagal, kembalikan Err("Failed to parse input.".to_string())
    // ...
    match s.parse::<i32>() {
        Ok(nilai) => Ok(nilai),
        Err(_) => Err("Failed to parse input.".to_string())
    }
}

fn add_numbers(a: &str, b: &str) -> Result<i32, String> {
    // TODO: Gunakan parse_number dan operator ? untuk menjumlahkan dua input string
    // ...
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    Ok(num_a + num_b)
}

fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // TODO: Kembalikan Some(nilai) jika index valid; jika tidak, kembalikan None
    // ...
    vec.get(index).copied()
}

pub fn test_handling_result_error_main() {
    let result = add_numbers("10", "20");
    match result {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => println!("{}", e),
    }

    let failed = add_numbers("10", "abc");
    match failed {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => println!("{}", e),
    }

    let data = vec![11, 22, 33, 99];
    let found = get_element(&data, 3);
    match found {
        Some(value) => println!("Found: Some({})", value),
        None => println!("No value at index"),
    }

    let not_found = get_element(&data, 10);
    match not_found {
        Some(value) => println!("Found: Some({})", value),
        None => println!("No value at index"),
    }
}
