#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Ćwiartka ze stanu {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Maksimum jest ustawione na {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("Ćwiartka ze stanu {:?}!", state);
    } else {
        count += 1;
    }
}
