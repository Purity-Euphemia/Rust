fn main() {
    let correct_pin: i32 = 1234;
    let entered_pin: i32 = 1234;

    if entered_pin == correct_pin {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}