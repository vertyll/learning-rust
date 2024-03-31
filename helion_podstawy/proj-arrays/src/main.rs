fn main() {
    let dni_tygodnia: [&str; 7] = [
        "Poniedziałek",
        "Wtorek",
        "Środa",
        "Czwartek",
        "Piątek",
        "Sobota",
        "Niedziela",
    ];

    println!("Dni tygodnia: {:?}", dni_tygodnia);
    println!("Dzisiaj mamy: {}", dni_tygodnia[1]);

    for dzien in dni_tygodnia.iter() {
        println!("Dzień: {}", dzien);
    }

    let mut wyniki: [i32; 5] = [1, 2, 3, 4, 5];
    wyniki[1] = 20;

    println!("Wyniki: {:?}", wyniki);

    const DOMYSLNA_WARTOSC: i32 = 0;
    let tablica_zer: [i32; 5] = [DOMYSLNA_WARTOSC; 5];
    println!("Tablica zer: {:?}", tablica_zer);
}
