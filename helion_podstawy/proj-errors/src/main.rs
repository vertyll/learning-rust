use std::fs::File;
use std::io::Read;

fn main() {
    // podziel(10, 0);

    let greetin_file_result = File::open("plik.txt");
    let mut greeting_file = match greetin_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    let mut contents = String::new();
    match greeting_file.read_to_string(&mut contents) {
        Ok(_) => println!("Zawartość pliku: {}", contents),
        Err(error) => panic!("Problem reading the file: {:?}", error),
    };
}

// fn podziel(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         panic!("Nie można dzielić przez 0");
//     }
//     a / b
// }
