fn double_number_with_match(input: &str) -> Result<i32, std::num::ParseIntError> {
    match input.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}

fn double_number_with_propagation(input: &str) -> Result<i32, std::num::ParseIntError> {
    // Gunakan ? untuk langsung return Err jika parse gagal
    let n = input.parse::<i32>()?;
    Ok(n * 2)
}

pub fn use_double_number() {
    println!("{:?}", double_number_with_match("10"));   // Ok(20)
    println!("{:?}", double_number_with_match("abc"));  // Err(ParseIntError { kind: InvalidDigit })

    println!("{:?}", double_number_with_propagation("10"));   // Ok(20)
    println!("{:?}", double_number_with_propagation("abc"));  // Err(ParseIntError { kind: InvalidDigit })
}