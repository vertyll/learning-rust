fn main() {
    println!("{0} ma psa a, {1} ma kota.", "Ala", "Mikołaj");

    println!("{name} ma {age} lat", name="Test", age="20");

    let number = 50;

    println!("Binarnie {:b}, Oktalnie: {:o}, Heksadecymalnie: {:x}", number, number, number);

    let list = vec!(1, 2, 3);
    println!("Lista: {:#?}", list);
}
