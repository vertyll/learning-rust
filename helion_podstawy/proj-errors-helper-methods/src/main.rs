use std::fs::File;
use std::io::Read;
use log::error;

fn main() {
    // let mut greeting_file = File::open("plik.txt").unwrap();
    //
    // let mut greeting_file = File::open("plik.txt")
    //     .expect("Nie udało się otworzyć pliku");
    //
    // let mut contents = String::new();
    // greeting_file.read_to_string(&mut contents)
    //     .expect("Problem reading the file");
    // println!("Zawartość pliku: {}", contents);

    let mut greeting_file = File::open("plik.txt")
        .unwrap_or_else(|error| {
            panic!("Problem opening the file: {:?}", error);
        });
    let mut contents = String::new();
    greeting_file.read_to_string(&mut contents)
        .unwrap_or(0);
    println!("Zawartość pliku: {}", contents);

    let mut greeting_file = File::open("plik.txt")
        .unwrap_or_else(|error| {
            panic!("Problem opening the file: {:?}", error);
        });
    let mut contents = String::new();
    greeting_file.read_to_string(&mut contents)
        .unwrap_or_else(|error| {
            panic!("Problem reading the file: {:?}", error);
        });
    println!("Zawartośc pliku: {}", contents);
}
