fn main() {
    fn najwiekszy<T: PartialOrd + Copy>(lista: &[T]) -> T {
        let mut najwiekszy: T = lista[0];

        for &item in lista.iter() {
            if item > najwiekszy {
                najwiekszy = item;
            }
        }
        najwiekszy
    }

    let lista = vec![34, 50, 25, 100, 65];
    let wynik = najwiekszy(&lista);
    println!("Największy element w {:?} to {}", lista, wynik);

    struct Punkt<T> {
        x: T,
        y: T,
    }

    let punkt_int = Punkt { x: 5, y: 10 };
    let punkt_float = Punkt { x: 5.5, y: 10.10 };
    println!("Punkt int_x: {}", punkt_int.x);
    println!("Punkt float_y: {}", punkt_float.y);

    impl<T> Punkt<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    let punkt = Punkt { x: 5, y: 10 };
    println!("x: {}", punkt.x());
    println!("y: {}", punkt.y());
}
