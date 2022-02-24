fn main() {
    let mut counter = 0;

    // loop + expressions that return a value
    let result = loop {
        counter += 1;
        println!("counter: {}", counter);

        if counter == 10 {
            break counter * 2;  // this returns a value
        }
    };
    println!("The result is {}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop (very used in Rust!)
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {  // catch the array length
        println!("the array value is: {}", element);
    }

    // the 'while' example (LIFTOFF!) with 'for'
    for number in (1..4).rev() {  // uses a 'Range'
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
