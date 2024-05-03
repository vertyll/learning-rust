use rand;
use rand::Rng;

fn main() {
    let mut secret = 0;

    println!("Your initial secret value is: {}", secret);

    secret = rand::thread_rng().gen_range(1..101);

    println!("Secret value after change: {}", secret);

    match secret {
        1 => println!("You win"),
        _ => println!("You lose!"),
    }
}
