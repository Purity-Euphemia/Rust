fn main() {
    let is_subscribed: bool = true;
    let has_paid: bool = true;

    if is_subscribed && has_paid {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}
