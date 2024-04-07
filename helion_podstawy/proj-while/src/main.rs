fn main() {
    let mut liczba = 0;
    while liczba < 5 {
        println!("Liczba: {}", liczba);
        liczba += 1;
    }

    let mut a = 0;
    let mut b = 5;
    while a < b {
        println!("a: {}, b: {}", a, b);
        a += 1;
        b -= 1;
    }

    let mut kontynuuj = true;
    let mut licznik = 0;
    while kontynuuj {
        if licznik > 3 {
            kontynuuj = false;
        }
        println!("Iteracja {}", licznik);
        licznik += 1;
    }

    while true {
        println!("Pętla nieskończona");
        break;
    }
}
