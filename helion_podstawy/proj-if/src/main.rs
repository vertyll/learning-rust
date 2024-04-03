fn main() {
    let liczba: i32 = 5;
    if liczba < 10 {
        println!("Liczba jest mniejsza niż 10");
    } else {
        println!("Liczba jest większa lub równa 10");
    }

    let warunek: bool = true;

    if warunek {
        println!("Wartość jest prawdziwa");
    }

    let liczba: i32 = 6;
    if liczba % 4 == 0 {
        println!("Liczba jest podzielna przez 4");
    } else if liczba % 3 == 0 {
        println!("Liczba jest podzielna przez 3");
    } else {
        println!("Liczba nie jest podzielna ani przez 4 ani przez 3");
    }

    let warunek: bool = true;
    let liczba: i32 = if warunek { 5 } else { 6 };
    println!("Wartość liczby: {}", liczba);
}
