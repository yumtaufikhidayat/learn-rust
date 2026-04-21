fn find_char(haystack: &str, needle: char) -> Option<usize> {
    // Iterasi tiap karakter beserta indeksnya
    for (index, c) in haystack.char_indices() {
        if c == needle {
            return Some(index); // ketemu, kembalikan indeks dalam Some
        }
    }
    None // jika loop selesai tanpa menemukan, kembalikan None
}

pub fn use_find_char() {
    let text = "Dicoding";
    let find = 'o';
    let result = find_char(text, find);

    // Menggunakan match untuk memeriksa Option
    match result {
        Some(pos) => println!("Karakter '{}' ditemukan di indeks {}.", find, pos),
        None => println!("Karakter '{}' tidak ditemukan.", find),
    }
}

pub fn option_extract_value_some() {
    let opt_val: Option<f32> = Some(3.5);

    // Menggunakan if let untuk mengambil nilai jika Some
    if let Some(x) = opt_val {
        println!("Nilai terdapat: {}", x);
    } else {
        println!("Tidak ada nilai.");
    }

    // Contoh if let lain: menghindari None
    let maybe_word: Option<&str> = None;
    if let Some(word) = maybe_word {
        println!("Kata: {}", word);
    } // tidak perlu else di sini jika tidak perlu melakukan apa-apa saat None
}

pub fn use_unwrap() {
    let maybe_number = Some(8);
    let no_number: Option<i32> = None;

    println!("{}", maybe_number.unwrap());
    // println!("{}", no_number.unwrap()); // panic! thread 'main' panicked at src/main.rs:6:30
}

pub fn use_expect() {
    let no_number: Option<i32> = None;
    println!("{}", no_number.expect("Nilai hilang")); // Panic! Nilai hilang!
}

pub fn unwrap_or() {
    let maybe_number = Some(8);
    let no_number: Option<i32> = None;

    println!("{}", maybe_number.unwrap_or(0)); // output: 8
    println!("{}", no_number.unwrap_or(0)); // output: 0
}

struct UserPreference {
    theme: String
}

fn compute_default() -> UserPreference {
    println!("computing ...");
    
    UserPreference {
        theme: String::from("VSCode")
    }
}

pub fn use_unwrap_or_else() {
    let maybe_user_preference = Some(UserPreference { theme: String::from("GitHub") });
    let no_user_preference: Option<UserPreference> = None;

    let pref_1 = maybe_user_preference.unwrap_or_else(|| compute_default());
    let pref_2 = no_user_preference.unwrap_or_else(|| compute_default());  // output: computing ...

    println!("{}", pref_1.theme);
    println!("{}", pref_2.theme);
}

pub fn use_map() {
    let maybe_number = Some(8);
    let no_number: Option<i32> = None;

    // Contoh unwrap_or: sediakan default jika None
    println!("maybe_number.unwrap_or(0) = {}", maybe_number.unwrap_or(0));
    println!("no_number.unwrap_or(0) = {}", no_number.unwrap_or(0));

    // Contoh map: kalikan nilai di dalam Option jika ada
    println!("maybe_number.map(*2) = {:?}", maybe_number.map(|v| v * 2));
    println!("no_number.map(*2) = {:?}", no_number.map(|v| v * 2));

    // Contoh expect: mengambil nilai atau panic dengan pesan custom
    let name: Option<&str> = Some("Dicoding");
    println!("Name: {}", name.expect("Name should be present"));
    // Jika `name` adalah None, program akan panic dengan pesan "Name should be present".
}