#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


mod front_of_house {
    /*
    * Inside modules, we can have other modules, as in this case with the modules hosting and serving
    * Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions (in this case)
    */

    /*
    * The pub keyword on a module only lets code in its ancestor modules refer to it.
    * The privacy rules apply to structs, enums, functions, and methods as well as modules.
    */
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*
* Choosing whether to use a relative or absolute path is a decision you’ll make based on your project
* Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other
*/

/*
* The 'fix_incorrect_order' function is in the 'back_of_house' module, 
* so we can use 'super' to go to the parent module of 'back_of_house', which in this case is 'crate', the root.
* From there, we look for 'serve_order' and find it
*/

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use self::front_of_house::hosting;
use self::front_of_house::hosting as HostingMod;

/*
* You might have wondered why we specified use crate::front_of_house::hosting and then called hosting::add_to_waitlist in eat_at_restaurant 
* rather than specifying the use path all the way out to the add_to_waitlist function to achieve the same result
* There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
*/

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    HostingMod::add_to_waitlist();
    
}

// pub use crate::front_of_house::hosting;  // pub use, could be reimported only once!

/*
* To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine 'pub' and 'use'
* This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope
*/
