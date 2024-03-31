fn main() {
    let x = plus_one(5);

    println!("Wartość x wynosi: {x}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("Wartość y wynosi: {y}");

    print_labeled_measurement(32, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Wynik pomiaru: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
