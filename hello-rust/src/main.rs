fn main() {
    println!("Hello, world!");
    let result = add(5, 3);
    println!("The sum is: {}", result);

    let nama_user = "Taufik"; // Ini tipenya &str
    let message = hello(nama_user);
    println!("{}", message);

    let message_to_show = "This is a message to show.";
    show_message(message_to_show);
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