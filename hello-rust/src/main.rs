mod ownership_borrowing;
mod enum_ip_address_kind;
mod user;
mod option;
mod result;
mod error_propagation; 
mod handling_result_error;
mod generic;
mod traits;
mod practice_traits_generic_lifetime;
mod modularization;
mod concurrency;

use ownership_borrowing::*;
use enum_ip_address_kind::IpAddressKind;
use enum_ip_address_kind::IpAddress;
use enum_ip_address_kind::IpAddresses;
use enum_ip_address_kind::TrafficLight;
use std::collections::HashMap;
use user::merge_all;
use option::use_find_char;
use option::option_extract_value_some;
use option::use_unwrap;
use option::use_expect;
use option::use_unwrap_or_else;
use option::use_map;
use result::use_result;
use result::use_custom_result;
use result::real_case_result;
use error_propagation::use_double_number;
use handling_result_error::test_handling_result_error_main;
use generic::use_generic;
use generic::use_generic_that_return_first_of_list;
use traits::use_traits_debug;
use traits::use_traits_display;
use traits::use_traits_eq_partial_eq;
use traits::use_traits_partial_ord_sort_students;
use traits::use_traits_clone;
use traits::use_traits_iterator_counter;
use traits::print_and_return_max;
use practice_traits_generic_lifetime::practice_traits_generic_lifetime_main;
use modularization::visibility::use_visibility_modularization;
use concurrency::use_concurrency;
use concurrency::use_concurrency_mpsc_channel;
use concurrency::use_concurrency_mutex;

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

    // use_enum();
    
    // route(IpAddressKind::V4);
    // route(IpAddressKind::V6);

    // traffic_light_enum();

    // use_hash_map();

    // merge_all();

    // use_find_char();
    // option_extract_value_some();
    // use_unwrap();
    // use_expect();
    // use_unwrap_or_else();
    // use_map();

    // use_result();
    // use_custom_result();
    // real_case_result();
    
    // use_double_number();

    // test_handling_result_error_main();

    // use_generic();
    // use_generic_that_return_first_of_list();

    // use_traits_debug();
    // use_traits_display();
    // use_traits_eq_partial_eq();
    // use_traits_partial_ord_sort_students();
    // use_traits_clone();
    // use_traits_iterator_counter();
    // print_and_return_max(&[1, 2, 3, 4, 5]);

    // practice_traits_generic_lifetime_main();

    // util::greet("Taufik");

    // use_visibility_modularization();

    use_concurrency();
    use_concurrency_mpsc_channel();
    use_concurrency_mutex();
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

fn enum_example() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
}

fn route(ip_kind: IpAddressKind) {
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
}

fn uses_enum_ip_addresses() {
    let home = IpAddresses::V4(String::from("127.0.0.1"));
    let loopback = IpAddresses::V6(String::from("::1"));
}

fn traffic_light_enum() {
    let light = TrafficLight::Yellow;
    
    match light {
        TrafficLight::Red => {
            println!("Lampu merah, berhenti!")
        }
        TrafficLight::Yellow => {
            println!("Lampu kuning, harap hati-hati.")
        }
        TrafficLight::Green => {
            println!("Lampu hijau, silakan jalan.")
        }
    
    }
}

fn use_hash_map() {
    let mut scores = HashMap::new();

    // Insert pasangan kunci-nilai ke HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    print!("Scores: {:?}", scores);

    // Overwrite nilai dengan kunci yang sama
    scores.insert(String::from("Blue"), 25);
    println!("Scores setelah tim Blue di-update: {:?}", scores);

    // Mengambil nilai berdasarkan kunci
    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("Skor tim {} adalah {}", team_name, score)
    }

    // Mencoba mengambil dengan kunci yang tidak ada
    let team_name_2 = String::from("Green");
    match scores.get(&team_name_2) {
        Some(_score) => println!("Skor tim {} adalah {:?}", team_name_2, scores),
        None => println!("Tim {} tidak ditemukan.", team_name_2),
    }
}

// Simpler way to use mod
mod util {
    // isi modul util, misal:
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
}