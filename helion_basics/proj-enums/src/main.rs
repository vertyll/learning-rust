#[allow(dead_code)]
fn main() {
    enum Kierunek {
        Polnoc,
        Poludnie,
        Wschod,
        Zachod,
    }
    let kierunek = Kierunek::Polnoc;

    match kierunek {
        Kierunek::Polnoc => println!("Idziemy na północ"),
        Kierunek::Poludnie => println!("Idziemy na południe"),
        Kierunek::Wschod => println!("Idziemy na wschód"),
        Kierunek::Zachod => println!("Idziemy na zachód"),
    }

    enum Pojazd {
        Samochod(String),
        Rower(String),
        Samolot(String, u32),
    }

    let pojazd = Pojazd::Samolot("Boeing 747".to_string(), 1_000);
    match &pojazd {
        Pojazd::Samochod(marka) => println!("Jadę samochodem marki {}", marka),
        Pojazd::Rower(marka) => println!("Jadę rowerem marki {}", marka),
        Pojazd::Samolot(marka, predkosc) => println!(
            "Lecę samolotem marki {} z prędkością {} km/h",
            marka, predkosc
        ),
    }

    impl Pojazd {
        fn opis(&self) -> String {
            match self {
                Pojazd::Samochod(model) => format!("Jadę samochodem model: {}", model),
                Pojazd::Rower(model) => format!("Jadę rowerem model: {}", model),
                Pojazd::Samolot(model, liczba_siedzen) => {
                    format!(
                        "Lecę samolotem: {} z liczbą siedzeń: {}",
                        model, liczba_siedzen
                    )
                }
            }
        }
    }
    println!("{}", pojazd.opis());
}
