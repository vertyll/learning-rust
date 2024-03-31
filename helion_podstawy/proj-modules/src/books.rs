pub struct Ksiazka {
    pub tytul: String,
    pub autor: String,
}

pub fn dodaj_ksiazke(tytul: String, autor: String) -> Ksiazka {
    Ksiazka { autor, tytul }
}
