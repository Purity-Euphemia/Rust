fn main() {
    let password_length: usize = 10;

    if password_length >= 8 {
        println!("Strong password");
    } else {
        println!("Weak password");
    }
}
