struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    // println!("the area of the rectangle is {}", calculate_area(&rectangle))
    println!("the area of the rectangle is {}", rectangle.area());
}

// fn calculate_area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }