pub struct KalkulatorHistoria {
    pub historia: Vec<String>,
}

impl KalkulatorHistoria {
    pub fn new() -> KalkulatorHistoria { KalkulatorHistoria { historia: Vec::new() }}

    pub fn dodaj(&mut self, a: i32, b: i32) -> i32 {
        let wynik = a + b;
        self.historia.push(format!("dodaj: {} + {} = {}", a, b, wynik));
        wynik
    }

    pub fn poziel(&mut self, a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Dzielenie przez 0!");
        }

        let wynik = a / b;
        self.historia.push(format!("podziel: {} / {} = {}", a, b, wynik));
        wynik
    }
}

#[cfg(test)]
mod tests {
    use super::KalkulatorHistoria;

    #[test]
    fn test_historia() {
        let mut kalkulator = KalkulatorHistoria::new();
        kalkulator.dodaj(2, 3);

        assert!(!kalkulator.historia.is_empty());
        assert_eq!(kalkulator.historia[0], "dodaj: 2 + 3 = 5");
    }

    #[test]
    #[should_panic(expected = "Dzielenie przez 0!")]
    fn test_panika_przy_dzieleniu_przez_zero() {
        let mut kalkulator = KalkulatorHistoria::new();
        kalkulator.poziel(10, 0);
    }
}