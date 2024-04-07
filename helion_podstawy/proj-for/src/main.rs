fn main() {
    for liczba in 1..5 {
        println!("Liczba: {}", liczba);
    }

    let wektor = vec![10, 20, 30, 40];
    for element in wektor {
        println!("Wektor: {}", element);
    }

    let wektor = vec!['a', 'b', 'c', 'd'];
    for (index, wartosc) in wektor.iter().enumerate() {
        println!("Index: {}, wartość: {}", index, wartosc);
    }

    for i in (1..5).rev() {
        println!("Liczba: {}", i);
    }

    for i in 1.. {
        println!("{}!", i);
        if i == 5 {
            break;
        }
    }

    loop {
        println!("Pętla nieskończona");
        break;
    }

    for i in 1.. {
        if i == 5 {
            continue;
        }
        println!("{}", i);
        if i == 10 {
            break;
        }
    }
}