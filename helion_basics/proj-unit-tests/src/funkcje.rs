pub fn dodaj(a: i32, b: i32) -> i32 {
    a + b
}

pub fn odejmij(a: i32, b: i32) -> i32 {
    a - b
}

pub fn funkcja_mozliwy_blad() -> Result<(), String> {
    Err("Jakiś błąd".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodaj() {
        assert_eq!(dodaj(2, 2), 4);
    }

    #[test]
    fn test_odejmij() {
        assert_eq!(odejmij(10, 5), 5);
    }

    #[test]
    fn test_funkcja_mozliwy_blad() {
        match funkcja_mozliwy_blad() {
            Ok(_) => panic!("Oczekiwano błędu ale funkcja zakończyła się sukcesem"),
            Err(e) => assert_eq!(e, "Jakiś błąd"),
        }
    }
}