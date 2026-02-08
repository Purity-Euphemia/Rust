fn main() {
    let score: u8 = 72;

    if score >= 70 {
        println!("Grade A");
    } else if score >= 60 {
        println!("Grade B");
    } else if score >= 50 {
        println!("Grade C");
    } else {
        println!("Fail");
    }
}
