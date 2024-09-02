#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado,
    New_Mexico,
    Virginia,
    Texas,
    Florida,
    Missisipi,
    Maine,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin { // Pattern => Code
        Coin::Penny => {
            println!("Pretty Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is from {state:?}!");
            25
        },
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }else {
        // code here
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // you may also use 'other' or '_' to represent all other cases
    }
} 
