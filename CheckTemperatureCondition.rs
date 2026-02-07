fn main() {
    let temperature: i32 = 30;

    if temperature > 35 {
        println!("Hot");
    } else if temperature >= 20 {
        println!("Warm");
    } else {
        println!("Cold");
    }
}
