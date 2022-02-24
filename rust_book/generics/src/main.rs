
// GENERICS

// function
fn myfn<T>(val: T) -> T {
    val
}
// struct
struct Point<T,U> {
    x: T,
    y: U,
}

// enum - 1
enum Option<T> {
    Some(T),
    None,
}

// enum - 2 
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// methods - 1
impl<T, U> Point<T, U> {
    fn x(&self) -> &T{
        &self.x
    }
}

// methods - 2
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// methods - 3
// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition.
impl <T,U> Point<T, U> {
    fn mixup<V,W> (self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    println!("Hello, world!");

    let p1 = Point { x: 5, y: 10.0 };

    println!("p.x = {}", p1.x());

    // println!("distance from origin: {}", p1.distance_from_origin());  // uncomment for getting a compilation error -> method not defined for <integer, float>

    let p2 = Point {x: 3.0, y: 4.0};
    println!("distance from origin: {}", p2.distance_from_origin());

    let p3 = Point {x: 3.0, y: "Str"};
    let p4 = p2.mixup(p3);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);

    // Performance of Code Using Generics
    /*
    * Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types by performing monomorphization
    * Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled
    */
}
