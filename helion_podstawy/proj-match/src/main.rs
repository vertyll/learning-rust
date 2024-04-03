fn main() {
    let liczba: i32 = 3;
    match liczba {
        1 => println!("Liczba jest równa 1"),
        2 => println!("Liczba jest równa 2"),
        3 => println!("Liczba jest równa 3"),
        _ => println!("Liczba jest równa {}", liczba),
    }

    enum Kierunek {
        Polnoc,
        Zachod,
        Poludnie,
        Wschod,
    }

    let kierunek = Kierunek::Poludnie;

    match kierunek {
        Kierunek::Polnoc => println!("Idziemy na polnoc"),
        Kierunek::Wschod => println!("Idziemy na wschod"),
        Kierunek::Poludnie => println!("Idziemy na poludnie"),
        Kierunek::Zachod => println!("Idziemy na zachod"),
    }

    let liczba: i32 = 6;
    match liczba {
        n if n % 2 == 0 => println!("Parzysta liczba"),
        n if n % 2 != 0 => println!("Liczba nie jest parzysta"),
        _ => println!("Coś innego"),
    }

    let liczba: i32 = 5;
    match liczba {
        1..=5 => println!("Liczba mieści się w przedziale od 1 do 5"),
        6..=10 => println!("Liczba mieści się w przedziale od 6 do 10"),
        _ => println!("Liczba nie mieści się w przedziale do 10"),
    }

    enum Kolor {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    let kolor = Kolor::Rgb(255, 255, 255);
    match kolor {
        Kolor::Rgb(r, g, b) => println!("Czerwony: {}, Zielony: {}, Niebieski: {}", r, g, b),
        Kolor::Hsv(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
    }

    let liczba = 5;
    let wynik = match liczba {
        1 => "jeden",
        2 => "dwa",
        _ => "inna liczba",
    };
    println!("Wynik: {}", wynik);
}
