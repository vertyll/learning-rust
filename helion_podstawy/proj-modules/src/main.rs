mod books;

fn main() {
    let book = books::dodaj_ksiazke("Pan tadeusz".to_string(), "Adam Mickiewicz".to_string());

    println!("Book: {}, Author: {}", book.tytul, book.autor);
    library::dodaj_magazyn();
}

pub mod library {
    pub fn dodaj_magazyn() {
        println!("Dodano magazyn");
    }
}
