fn main() {
    let number = 7;
    let thres = 5;

    // if/else expressions work only on boolean types!
    if number < thres {
        println!("number {} is < than {}", number, thres);
    } else {
        println!("number {} is >= than {}", number, thres);
    }

    // else if, a lot of else if can clutter your code -> match
    if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4, 3, or 2", number);
    }

    // uaing if in a let statement
    let condition = false;
    let number = if condition { number } else { - number };  // types should not be mismatched in each if 'arm'

    println!("The value of number, given the condition ({}) is: {}", condition, number);
}
