#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    Rectangle::square(3);

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych.",
        rect1.area()
    );

    println!(
        "Czy rect2 zmieści się wewnątrz rect1? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Czy rect3 zmieści się wewnątrz rect1? {}",
        rect1.can_hold(&rect3)
    );
}
