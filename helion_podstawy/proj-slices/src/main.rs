fn main() {
    let tablica = [1, 2, 3, 4, 5];
    let slice = &tablica[1..4];
    println!("Slice: {:?}", slice);

    let mut tablica = [1, 2, 3, 4, 5];
    println!("Tablica przed mutacją: {:?}", tablica);
    let slice_mut = &mut tablica[1..4];
    slice_mut[0] = 6;
    println!("Slice mut: {:?}", slice_mut);
    println!("Tablica po mutacji: {:?}", tablica);

    let tablica = [1, 2, 3, 4, 5];
    fn sumuj(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    let suma = sumuj(&tablica[1..4]);
    println!("Suma: {}", suma);
}
