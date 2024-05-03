use crate::moja_matematyka::matematyka::dodaj;

mod moja_matematyka;

fn main() {
    println!("Suma: {}", dodaj(5, 5));

    let rn = rand::random::<f64>();
    println!("Random number: {}", rn);
}
