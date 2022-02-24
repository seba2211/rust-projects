use std::io;

fn main() {
    // 3.1 - 1
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    

    // 3.1 - 2
    
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    

    // 3.2 - 1 - floating points
    let x = 2.0; // f64, double precision

    let y: f32 = 3.0; // f32, single-precision float

    println!("x = {} is double-precision, y = {} is single-precision", x, y);

    // numeric operation and order remains as in C

    // 4 - booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t = {} and f = {} are booleans", t, f);

    //

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
