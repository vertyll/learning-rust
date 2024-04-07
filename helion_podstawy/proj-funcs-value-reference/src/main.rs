fn main() {
    let x = 10;
    kopia(x);
    println!("Oryginalna wartość {}", x);

    let mut x = 10;
    println!("Oryginalana wartość: {}", x);
    zmien(&mut x);
    println!("Zmodyfikowana wartość: {}", x);

    let mut v = vec![1, 2, 3];
    v = wypisz_i_zwroc(v);
    println!("Wartość wektora: {:?}", v);

    let mut v = vec![1, 2, 3];
    dodaj_element(&mut v);
    println!("{:?}", v);
}

fn kopia(mut liczba: i32) {
    // Tutaj 'liczba' jest kopią wartośći przekazanej do funkcji
    liczba += 10;
    println!("Wartość w funkcji: {}", liczba);
}

fn zmien(liczba: &mut i32) {
    *liczba += 10;
}

fn wypisz_i_zwroc(wektor: Vec<i32>) -> Vec<i32> {
    for element in &wektor {
        println!("{}", element);
    }
    wektor
}

fn dodaj_element(wektor: &mut Vec<i32>) {
    wektor.push(4);
}

