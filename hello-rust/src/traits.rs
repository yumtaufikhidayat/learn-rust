use std::fmt;
use std::collections::VecDeque;
use serde::Serialize;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn use_traits_debug() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);       // Debug print
}


// Implementasi trait Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn use_traits_display() {
    let p = Point { x: 3, y: 4 };
    println!("{}", p);  // Menggunakan Display
}

pub fn use_traits_eq_partial_eq() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = 10;

    println!("a == b? {}", a == b);
    println!("a == c? {}", a == c);
    println!("a != b? {}", a != b);
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Student {
    name: String,
    score: u32,
}

pub fn use_traits_partial_ord_sort_students() {
    let mut students = vec![
        Student { name: String::from("Andi"), score: 85 },
        Student { name: String::from("Budi"), score: 92 },
        Student { name: String::from("Citra"), score: 78 },
    ];

    students.sort(); // Menggunakan trait Ord

    for s in students {
        println!("{:?}", s);
    }
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    pages: u32,
}

pub fn use_traits_clone() {
    let original_book = Book {
        title: String::from("Rust untuk Pemula"),
        pages: 300,
    };

    let copied_book = original_book.clone(); // Mengkloning objek

    println!("Asli: {:?}", original_book);
    println!("Salinan: {:?}", copied_book);
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32; // associated type untuk iterasi Counter

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn use_traits_iterator_counter() {
    let mut counter = Counter::new(5);

    while let Some(n) = counter.next() {
        println!("Hitungan: {}", n);
    }
}

struct Tweet {
    username: String,
    content: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle { width: f64, height: f64 }

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Ini AKAN ERROR!
// impl Serialize for VecDeque<String> {
//     fn serialize(&self, ... ) -> ... {
//     // implementasi
//     }
// }

// Trait bounds pada Generics
// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     // ... logic mencari elemen terbesar ...
// }

// Trait bounds pada Generics dengan Multiple Trait
pub fn print_and_return_max<T: PartialOrd + std::fmt::Display>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    println!("Max value: {}", max);
    max
}