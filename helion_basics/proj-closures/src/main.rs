fn main() {
    let powiedz_witaj = || println!("Witaj świecie");
    powiedz_witaj();

    let podwoj = |x| x * 2;
    let wynik = podwoj(5);
    println!("Wynik: {}", wynik);

    let opis_liczby = |x| {
        if x < 5 {
            "mniejsze niż 5"
        } else {
            "większa lub równa 5"
        }
    };
    let opis = opis_liczby(6);
    println!("Opis: {}", opis);

    let liczby = vec![1, 2, 3, 4];
    let liczby_plus_jeden: Vec<_> = liczby
        .into_iter()
        .map(|x| x + 1)
        .collect();
    println!("Liczby po dodaniu 1: {:?}", liczby_plus_jeden);

    let liczby = vec![1, 2, 3, 4, 5, 6];
    let tylko_parzyste: Vec<_> = liczby
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Tylko parzyste liczby: {:?}", tylko_parzyste);

    let liczby = vec![1, 2, 3, 4];
    let liczby_plus_jeden: Vec<_> = liczby
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x + 1)
        .collect();
    println!("Liczby parzyste po dodaniu 1: {:?}", liczby_plus_jeden);
}
