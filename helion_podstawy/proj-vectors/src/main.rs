fn main() {
    let mut liczby: Vec<i32> = Vec::new();
    println!("Pusty wektor {:?}", liczby);
    let numbers = vec![1, 2, 3];
    println!("Wektor z początkowymi wartościami: {:?}", numbers);
    let wektor_zer: Vec<i32> = vec![0; 10];
    println!("Wektor z 10 zerami: {:?}", wektor_zer);

    liczby.push(1);
    liczby.push(2);
    liczby.push(3);
    println!("Wektor po dodaniu liczb: {:?}", liczby);

    liczby.pop();
    println!("Wartość po dodaniu i usunięciu wektoru: {:?}", liczby);

    let pierwszy = liczby[0];
    println!("Pierwszy element wektoru: {:?}", pierwszy);

    if let Some(pierwszy) = liczby.get(0) {
        println!("Pierwszy element to: {:?}", pierwszy);
    }

    if let Some(pierwszy) = liczby.get(0..2) {
        println!("Pierwsze dwa elementy to {:?}", pierwszy);
    }

    for liczba in &liczby {
        println!("Liczba: {:?}", liczba);
    }

    for liczba in liczby.iter() {
        println!("Liczba: {:?}", liczba);
    }

    for liczba in &mut liczby {
        *liczba += 50;
    }
    println!("Wektor po dodaniu 50 do kazdego elementu: {:?}", liczby);
}
