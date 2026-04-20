/*
    Tugas:
    Lengkapi struktur dan fungsi berikut agar:
    - Struct `User` memiliki field `name: String` dan `age: u8`.
    - Enum `Status` berisi varian `Active`, `Inactive`, dan `Banned`.
    - Fungsi `describe_user` menerima struct dan enum lalu mencetak deskripsi sesuai status.
    - Fungsi `calculate_discount` menerima umur dan mengembalikan nilai diskon (10 jika umur >= 18, jika tidak 0).
    - Fungsi `handle_discount` mencetak pesan berdasarkan hasil dari `calculate_discount`.

    Contoh:
    User { name: "Alice", age: 20 }, Status::Active -> 
    "Alice (20) is Active"
    -> "Discount: 10%"

    User { name: "Bob", age: 12 }, Status::Banned ->
    "Bob (12) is Banned"
    -> "No discount"
*/

struct User {
    name: String,
    age: u8
}

enum Status {
    Active, Inactive, Banned
}

fn describe_user(user: &User, status: &Status) {
    let status_user = match status {
        Status::Active => "Active",
        Status::Inactive => "Inactive",
        Status::Banned => "Banned"
    };
    println!("{} ({}) is {}" , user.name, user.age, status_user)
}

fn calculate_discount(age: u8) -> i32 {
    if age >= 18 {
        18
    } else {
        0
    }
}

fn handle_discount(age: u8) {
    let discount = calculate_discount(age);
    if discount > 0 {
        println!("Discount: {}%", discount);
    } else {
        println!("No discount");
    }
}

pub fn merge_all() {
    let user1 = User {
        name: String::from("Alice"),
        age: 20
    };
    let status1 = Status::Active;
    describe_user(&user1, &status1);
    calculate_discount(user1.age);
    handle_discount(user1.age);

    let user2 = User {
        name: String::from("Bob"),
        age: 12
    };
    let status2 = Status::Banned;
    describe_user(&user2, &status2);
    calculate_discount(user2.age);
    handle_discount(user2.age);
}