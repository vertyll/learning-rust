fn main() {
    let tuple = ("Rust", 2021, true);
    println!("Tuple: {:?}", tuple);

    let nazwa = tuple.0;
    let rok = tuple.1;
    println!("Język: {}, z roku: {}", nazwa, rok);

    let (nazwa, rok, status) = tuple;
    println!("Język: {}, z roku: {}", nazwa, rok);

    fn podaj_informacje() -> (&'static str, i32) {
        ("Rust", 2021)
    }

    let infomacje = podaj_informacje();
    println!("Język: {}, z {} roku", infomacje.0, infomacje.1);

    let max_big_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("Tuple: {:?}", max_big_tuple);
}
