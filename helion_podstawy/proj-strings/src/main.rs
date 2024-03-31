fn main() {
    let nazwa_kota: &str = "Puszek";
    let nazwa_kota: &'static str = "Puszek";
    println!("Nazwa kota to: {}", nazwa_kota);

    let mut pies: String = String::new();
    pies = String::from("Max");
    println!("Nazwa psa to: {}", pies);

    let wlasciciel: String = format!("Cześć, jestem {} i jestem właścicielem psa", "Mikolaj");
    println!("{}", wlasciciel);

    let pies_zmiana: String = pies.replace("Max", "Burek");
    println!("{}", pies_zmiana);
}
