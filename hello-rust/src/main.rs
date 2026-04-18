mod ownership_borrowing;
mod enum_ip_address_kind;

use ownership_borrowing::*;
use enum_ip_address_kind::IpAddressKind;


fn main() {
    // println!("Hello, world!");
    // let result = add(5, 3);
    // println!("The sum is: {}", result);

    // let nama_user = "Taufik"; // Ini tipenya &str
    // let message = hello(nama_user);
    // println!("{}", message);

    // let message_to_show = "This is a message to show.";
    // show_message(message_to_show);

    // let radius: f32 = 4.0;
    // let luas = luas_lingkaran(radius);

    // if radius > 0.0 {
    //     println!("Luas lingkaran dengan jari-jari {} adalah {}", radius, luas);
    // } else {
    //     println!("Jari-jari harus lebih besar dari 0");
    // }

    // loop_sample([1, 2, 3, 4, 5]);
    // match_sample(radius);

    use_enum();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn hello(name: &str) -> String {
    format!("Hello {}, welcome to the world", name)
}

fn show_message(message: &str) {
    println!("{}", message);
}

// Lingkaran
// TODO: Buatlah konstanta PI dengan tipe f32 dan nilai awal 3.14
const PI: f32 = 3.14;

// TODO: Buatlah fungsi untuk menghitung luas lingkaran
fn luas_lingkaran(r: f32) -> f32 {
	return PI * r * r;
}

fn loop_sample(a: [i32; 5]) {
    // for x in a {
    //   println!("{}", x);
    // }

    for i in 1..=5 {
        println!("{}", i);
    }
}

fn match_sample(radius: f32) {
     // TODO: Lakukan match untuk mencetak kategori berdasarkan nilai radius
    // - Jika `radius` <= 5.0, cetak "Kecil".
    // - Jika `radius` <= 10.0, cetak "Sedang".
    // - Jika lebih dari 10.0, cetak "Besar".
    match radius {
      r if r <= 5.0 => println!("Kecil"),
      r if r <= 10.0 => println!("Sedang"),
      _ => println!("Besar")
    }
}

/*
* Expected output:
* 50.24
* 1
* 2
* 3
* 4
* 5
* Kecil
* 
*/

fn use_enum() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    route_ip_address(four);
    route_ip_address(six);
}

fn route_ip_address(ip_kind: IpAddressKind) {
    match ip_kind {
        IpAddressKind::V4 => println!("Routing IPv4 address"),
        IpAddressKind::V6 => println!("Routing IPv6 address"),
    }
}