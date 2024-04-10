use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let s = otworz_plik("plik.txt");
    println!("{:?}", s);
}

// fn otworz_plik_2(nazwa: &str) -> Result<String, io::Error> {
//     let mut f = match File::open(nazwa) {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn otworz_plik(nazwa: &str) -> Result<String, io::Error> {
    let mut f = File::open(nazwa)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}