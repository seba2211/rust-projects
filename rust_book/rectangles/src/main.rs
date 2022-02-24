// 5.2 - An Example Program Using Structs

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // calculate the area of the rectangle
        self.width * self.height
    }

    fn can_hold(&self, other_rect : &Rectangle) -> bool {  // check if 'self' can hold another rectangle
        if other_rect.area() < self.area() { // self 'can hold' the other_rect arg
            true
        } else {false}

        // self.width > other.width && self.height > other.height  // book implementation
    }

    fn square(size : u32) -> Rectangle {  // create a new element which is a square
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // using normal variables
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {0} square pixels [variables]", area1(width1, height1));

    // using tuples
    let dim = (width1, height1);
    println!("The area of the rectangle is {0} square pixels [tuples]", area2(dim));

    // using structs
    let rect1 = Rectangle {
        width: width1,
        height: height1
    };

    println!("The area of the rectangle is {0} square pixels [structs]", area3(&rect1));

    // Adding Useful Functionality with Derived Traits

    println!("rect1 is {:#?}", rect1);  // derive debug needed, pretty print with :#?

    // Method Syntax

    println!("The area of the rectangle is {0} square pixels [method]", rect1.area());

    /*
    * Rust doesn’t have an equivalent to the -> operator; 
    * instead, Rust has a feature called automatic referencing and dereferencing. 
    * Calling methods is one of the few places in Rust that has this behavior.
    */

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Create a square object of size 10
    let sq = Rectangle::square(10);
    println!("Square area:  {}", sq.area());
    println!("Square object -> {:#?}", sq);
}

fn area1(width : u32, height : u32) -> u32 {
    width * height
}

fn area2(dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle : &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


