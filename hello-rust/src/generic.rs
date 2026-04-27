fn jumlah_dua<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

pub fn use_generic() {
    let a = 2;
    let b = 3;

    println!("Hasil penjumlahan {} dan {} adalah: {}", a, b, jumlah_dua(a, b));
}

fn first<T>(list: &[T]) -> &T {
    &list[0]
}

pub fn use_generic_that_return_first_of_list() {
    let numbers = [10, 20, 30];
    let words = ["rust", "borrow", "ownership"];

    let num = first(&numbers);
    let word = first(&words);

    println!("First number: {}", num);
    println!("First word: {}", word);
}

// Stuct & Enum in generic
struct Point<T> {
    x: T,
    y: T,
}

fn use_point() {
    let point_int = Point { x: 5, y: 10 };      // T deduced as i32
    let point_float = Point { x: 1.5, y: 4.0 }; // T deduced as f64
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}