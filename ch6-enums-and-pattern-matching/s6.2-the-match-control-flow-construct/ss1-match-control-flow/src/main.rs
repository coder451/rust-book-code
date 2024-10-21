enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// The code here borrows the value so what it borrows remains valid
// after the call to value, which obviously does not mutate anything,
// or consume it in any permanent way
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_with_scopes(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {1},
        Coin::Nickel => {5},
        Coin::Dime => {10},
        Coin::Quarter => {25},
    }
}

fn main() {
    println!("The match Control Flow Construct");
    let penny = Coin::Penny;
    println!("Value of penny is {}", value_in_cents(&penny));
    println!("Value of nickel is {}", value_in_cents(&Coin::Nickel));
    println!("Value of dime is {}", value_in_cents(&Coin::Dime));
    println!("Value of quarter is {}", value_in_cents(&Coin::Quarter));

    println!("Value of penny is {}", value_in_cents_with_scopes(&penny));
}
