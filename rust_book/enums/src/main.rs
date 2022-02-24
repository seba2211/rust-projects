
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn val_to_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("It is a penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("It is a quarter coin from {:#?}", state);
                25
            },
        }
    }
}

fn val_to_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn main() {
    println!("Hello, world!");
    let m = Message::Write(String::from("Hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    match m {
        Message::Write(msg_write) => println!("Is a Write, with content: {}", msg_write),
        _ => println!("It is not a Write!"),
    }

    let coin = Coin::Quarter(UsState::Alabama);

    println!("The coin value in cents is {} [function]", val_to_cents(&coin));
    println!("The coin value in cents is {} [method]", coin.val_to_cents());

    let x = pw2(&Some(10));
    println!("x = {:?}", x);
    let y = pw2(&x);
    println!("y = {:?}", y);
    let z = pw2(&None);
    match z {
        Some(num) => println!("z = {}", num),
        None => println!("Value is None!"),
    }

    if let Coin::Quarter(_) = coin {
        println!("if let -> the coin is a quarter");
    } else {
        println!("if let -> the coin is not a quarter")
    }
}

fn pw2(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num*num),
    }
}