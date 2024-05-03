macro_rules! powitaj {
    () => {
        println!("Witaj świecie");
    };

    ($name: expr) => {
        println!("Witaj, {}!", $name);
    };
}

macro_rules! stworz_wektor {
    ( $( $x: expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $ (
                temp_vec.push($x);
            ) *
            temp_vec
        }
    };
}

macro_rules! zdefiniuj_funkcje {
    ($nazwa_funckji: ident) => {
        fn $nazwa_funckji() {
            println!(concat!("Funkcja ", stringify!($nazwa_funckji), " została wywołana."));
        }
    };
}

fn main() {
    powitaj!();
    powitaj!("Jan");

    let wektor = stworz_wektor![1, 2, 3, 4];
    println!("{:?}", wektor);

    zdefiniuj_funkcje!(moja_funkcja);
    moja_funkcja();
}