pub mod defs;  // defines (constants)

use defs::Guess as Guess;

impl Guess
{
    pub fn value_print(&self) {
        println!("My internal value is {}", self.value());
    }

    pub fn modify_value(&mut self, new_value : i32){
        if !Guess::check_value(new_value) {
            Guess::my_panic(new_value);
        }
        self.value = new_value;
    }
}
