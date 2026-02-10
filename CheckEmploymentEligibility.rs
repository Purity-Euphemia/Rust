fn main() {
    let age: u8 = 30;
    let has_id_card: bool = true;

    if age >= 18 && has_id_card {
        println!("Eligible for employment");
    } else {
        println!("Not eligible for employment");
    }
}
