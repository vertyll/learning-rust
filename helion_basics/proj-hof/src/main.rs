fn main() {
    let podwoj = |x| x * 2;
    let wynik = apply(podwoj, 5);
    println!("Wynik: {}", wynik);

    let liczby = vec![1, 2, 3, 4, 5];
    let wynik = przetwarzaj(
        liczby,
        |x| x % 2 == 0,
        |x| x * 2,
    );
    println!("Parzyste liczby podwojone: {:?}", wynik);

    let liczby = vec![1, 2, 3, 4, 5];
    let wynik = przetwarzaj_2(
        liczby,
        |x| x % 2 == 0,
        |x| x * 2,
    );
    println!("Parzyste liczby podwojone: {:?}", wynik);

    let leniwa_wartosc = |x| {
        println!("Obliczam...");
        x * x
    };
    println!("Przed obliczeniem");
    let wynik = leniwa_wartosc(4);
    println!("Po obliczeniu: {}", wynik);
}

fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
{
    f(value)
}

fn przetwarzaj(liczby: Vec<i32>, filtr: fn(&i32) -> bool, transofrmacja: fn(i32) -> i32) -> Vec<i32> {
    liczby.into_iter().filter(filtr).map(transofrmacja).collect()
}

fn przetwarzaj_2<F, G>(liczby: Vec<i32>, filttr: F, transofrmacja: G) -> Vec<i32>
    where
        F: Fn(&i32) -> bool,
        G: Fn(i32) -> i32,
{
    liczby
        .into_iter()
        .filter(filttr)
        .map(transofrmacja)
        .collect()
}
