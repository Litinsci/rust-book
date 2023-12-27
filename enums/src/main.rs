enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let penny = Coin::Penny;
    let nickels = Coin::Nickel;
    let dimes = Coin::Dime;
    let quarters = Coin::Quarter;
    println!("is {}", value_in_cents(penny));
    println!("is {}", value_in_cents(nickels));
    println!("is {}", value_in_cents(dimes));
    println!("is {}", value_in_cents(quarters));
}
