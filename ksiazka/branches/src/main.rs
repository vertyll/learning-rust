fn main() {
    let number = 3;

    if number < 5 {
        println!("warunek został spełniony");
    } else {
        println!("warunek nie został spełniony");
    }

    let condition = true;
    let number: u32 = if condition { 5 } else { 6 };

    println!("Wartość zmiennej number wynosi: {number}");
}
