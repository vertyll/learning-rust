fn main() {
    struct Osoba {
        imie: String,
        wiek: u32,
    }

    let osoba = Osoba {
        imie: String::from("Ania"),
        wiek: 24,
    };
    println!("{} ma {} lat", osoba.imie, osoba.wiek);

    struct Punkt(i32, i32);
    let punkt = Punkt(10, 20);
    println!("Punkt ma współrzędne: ({}, {})", punkt.0, punkt.1);

    impl Osoba {
        fn przedstaw_sie(&self) {
            println!("Mam na imię {} i mam {} lata", self.imie, self.wiek);
        }

        fn stworz_nieznane() -> Osoba {
            Osoba {
                imie: String::from("Nieznane"),
                wiek: 0,
            }
        }
    }
    osoba.przedstaw_sie();

    impl Default for Osoba {
        fn default() -> Self {
            Osoba {
                imie: String::from("Nieznane"),
                wiek: 0,
            }
        }
    }
    let osoba2 = Osoba::default();
    osoba2.przedstaw_sie();

    // impl Osoba {}
    let osoba3 = Osoba::stworz_nieznane();
    osoba3.przedstaw_sie();
}
