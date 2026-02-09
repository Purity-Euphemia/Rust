fn main() {
    let age: u8 = 16;

    if age < 13 {
        println!("Child");
    } else if age < 18 {
        println!("Teenager");
    } else {
        println!("Adult");
    }
}
