fn main() {

    // expressions do not use semicolons! (even inside functions)
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let z = plus_one(x);

    println!("The value of z is: {}", z);
}

fn five() -> i32 {
    5
}

fn plus_one(x : i32) -> i32 {
    x + 1
}