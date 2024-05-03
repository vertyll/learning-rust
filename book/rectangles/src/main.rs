// normalnie

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "Pole prostokąta wynosi: {} pikseli kwadratowych",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// krotki

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "Pole prostokąta wynosi: {} pikseli kwadratowych",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struktury

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // println!("rect1 to: {:#?}", rect1);

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
