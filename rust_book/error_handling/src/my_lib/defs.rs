pub const MIN_LIM : i32 = 1;
pub const MAX_LIM : i32 = 100;

pub struct Guess {
    pub value : i32,
}

impl Guess {

    pub fn new(value : i32) -> Guess {
        if !Guess::check_value(value) {
            Guess::my_panic(value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn check_value(value : i32) -> bool { 
        if value < MIN_LIM || value > MAX_LIM {
            return false
        }
        return true
    }

    pub fn my_panic(value : i32) {
        panic!("Guess value should be between {} and {}. Received {}", MIN_LIM, MAX_LIM, value);
    }
}