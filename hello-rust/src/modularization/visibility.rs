mod calculations {
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn area(width: i32, length: i32) -> i32 {
        multiply(width, length)
    }

    pub fn circumference(width: i32, length: i32) -> i32 {
        multiply(2, sum(width, length))
    }
}

pub fn use_visibility_modularization() {
    // Memanggil fungsi publik add() -> berhasil
    println!("{}", calculations::area(5, 3));
    println!("{}", calculations::circumference(5, 3));

    // Memanggil fungsi multiply() -> error, karena multiply tidak pub
    // println!("{}", calculations::multiply(5, 3));
}