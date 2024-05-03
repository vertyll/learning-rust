pub struct Kalkulator;

impl Kalkulator {
    pub fn dodaj(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn odejmij(a: i32, b:i32) -> i32 {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::Kalkulator;

    struct TestPrzypadek {
        a: i32,
        b: i32,
        oczekiwane_dodaj: i32,
        oczekiwane_odejmij: i32,
    }

    #[test]
    fn test_dodaj_i_odejmij() {
        let testy = vec![
            TestPrzypadek { a: 1, b: 2, oczekiwane_dodaj: 3, oczekiwane_odejmij: -1 },
            TestPrzypadek { a: 0, b: 0, oczekiwane_dodaj: 0, oczekiwane_odejmij: 0 },
        ];

        for test in testy {
            assert_eq!(Kalkulator::dodaj(test.a, test.b), test.oczekiwane_dodaj);
            assert_eq!(Kalkulator::odejmij(test.a, test.b), test.oczekiwane_odejmij);
        }
    }
}