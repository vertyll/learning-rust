fn main() {
    hello();
    hello2();

    let liczba = 5;
    let wynik = czy_parzysta(liczba);
    println!("Czy liczba {} jest parzysta {}", liczba, wynik);

    let liczba = 4;
    let wynik = czy_parzysta_2(liczba);
    println!("Czy liczba {} jest parzysta {}", liczba, wynik);

    let liczba = 2;
    let wynik = czy_parzysta_3(liczba);
    println!("Czy liczba {} jest parzysta {}", liczba, wynik);
}

fn hello() {
    println!("Witaj świecie");
}

fn hello2() {
    println!("Witaj świecie")
}

fn czy_parzysta(liczba: i32) -> bool {
    return liczba % 2 == 0;
}

fn czy_parzysta_2(liczba: i32) -> bool {
    return liczba % 2 == 0
}

fn czy_parzysta_3(liczba: i32) -> bool {
    liczba % 2 == 0
}