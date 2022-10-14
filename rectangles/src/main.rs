#![allow(unused)]
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let sq = Rectangle::square(3);
    println!(
        "\t Square\n
         Height: {}
         Width: {}
         Area: {}",
         sq.height, sq.width, sq.area()
    );

    {
        let scale = 2;
        let rec2 = Rectangle {
            width: dbg!(&rec1.width * scale),
            ..rec1
        };

        dbg!(&rec2.area());
        dbg!(&rec2);

        println!("Is rec1 bigger than rec2? {}", rec1.is_bigger(&rec2));
    }

    println!(
        "The area of the rectangle is {} square pixels",
        rec1.area()
    );
    dbg!(&rec1);


}

fn original_main() {
    let width1 = 30;
    let height1 = 50;
    let rec1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        new_area(rec1)  
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn new_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
