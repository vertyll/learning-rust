use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Zgadnij numer!");

    loop {
        println!("Poda swoją liczbę:");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Błąd wczytywania linii");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Wybrana przez Ciebie liczba to: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mała!"),
            Ordering::Greater => println!("Za duza!"),
            Ordering::Equal => {
                println!("Jesteś zwyciezca!");
                break;
            }
        };
    }
}
