#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size:u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let sq1 = Rectangle::square(5);

    dbg!(sq1);

    dbg!(&rect1);

    println!("Rect: {:?}", rect1);

    println!("The area of the rectangle is {0}", rect1.area());
    println!("The rectangle has a non-zero width: {}", rect1.width());

    println!("Can 1 fit 2? {}", rect1.can_fit(&rect2));
    println!("Can 2 fit 1? {}", rect2.can_fit(&rect1));
}

/* 
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/
