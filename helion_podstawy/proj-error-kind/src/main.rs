use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    let f = otworz_plik("plik.txt");
    match f {
        Ok(_) => println!("Plik otwarty pomyślnie"),
        Err(e) => println!("Wystąpił błąd: {}", e),
    }
}

fn otworz_plik(nazwa: &str) -> Result<(), io::Error> {
    match File::open(nazwa) {
        Ok(_) => {
            println!("Plik otwarty pomyślnie");
            Ok(())
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Plik nie został znaleziony, próba utworzenia nowego");
                File::create(nazwa)?;
                Ok(())
            },
            ErrorKind::PermissionDenied => {
                println!("Brak uprawnień do otwarcia pliku");
                Err(e)
            },
            _ => {
                println!("Wystąpił inny błąd");
                Err(e)
            },
        },
    }
}