use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Podaj wartość indexu tablicy: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Błąd przy wczytywaniu wartości");

    let index: usize = index
        .trim()
        .parse()
        .expect("Wprowadzona wartość to nie jest numer");

    let element = a[index];

    println!("Wartość elementu pod indexem: {index} wynosi: {element}");
}
