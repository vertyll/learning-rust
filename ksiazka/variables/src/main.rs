fn main() {
    let x = 5;

    let x = x + 1; //6

    {
        let x = x * 2; // x = 6*2 = 12
        println!("Wartość x w wewnętrznym bloku wynosi: {x}"); //12
    }

    println!("Wartość x wynosi: {x}"); //6
}
