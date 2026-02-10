fn main() {
    let first_value: i32 = 10;
    let second_value: i32 = 25;
    let third_value: i32 = 15;

    if first_value >= second_value && first_value >= third_value {
        println!("First value is largest");
    } else if second_value >= third_value {
        println!("Second value is largest");
    } else {
        println!("Third value is largest");
    }
}
