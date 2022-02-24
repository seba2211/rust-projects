mod lib;  // bring lib into scope
use lib::{Tweet, Summary};

struct ExStruct {
    pub content: String,
    pub author: String,
}

struct ExStruct2 {
    pub content: String,
    pub author: String,
}

/*
* One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate
*/


impl Summary for ExStruct {
    fn summarize(&self) -> String {
        format!("{}: {}  -> but this is an example structure of an outside struct implementing the trait in lib.rs", self.author, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("# {} #", self.author)
    }
}

impl Summary for ExStruct2 {  // implement the default implementation of the Summary::summarize trait for ExStruct2
    fn summarize_author(&self) -> String {
        format!("-> {} <-", self.author)
    }
}

/*
* Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.
*/

fn main()
{
    println!("Hello world -> traits");

    // Traits - 1
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let ex_struct = ExStruct {
        content: String::from("Example content"),
        author: String::from("Me"),
    };

    println!("first example structure: {}", ex_struct.summarize());

    // Default implementation
    let ex_struct2 = ExStruct2 {
        content: String::from("Example content 2"),
        author: String::from("Me again!"),
    };

    println!("second example structure: {}  -> default implementation for summarize() ", ex_struct2.summarize());

    // Traits as Parameters
    println!("-> Traits as parameter");
    lib::notify(&tweet);

    // Returning Types that Implement Traits
    println!("-> Returning Types that Implement Traits");
    println!("{}", lib::returns_summarizable().summarize());

    // Fixing the largest Function with Trait Bounds
    println!("-> Fixing the largest Function with Trait Bounds");
    let my_vec = vec![-100, -1000, 1, 200, 20];
    println!("The largest element in the list is {}", largest(&my_vec));

    println!("The largest element [WITHOUT COPY TRAIT] in the list is  {}", largest_no_copy_trait(&my_vec));

    // Using Trait Bounds to Conditionally Implement Methods

    let my_pair = lib::Pair::new(300, 200);
    my_pair.cmp_display();

    let my_pair = lib::Pair::new('a', 'b');
    my_pair.cmp_display();

    let my_pair = lib::Pair::new("hello", "world");
    my_pair.cmp_display();

    let my_pair = lib::Pair::new(vec![1,2], vec![3,4]);  
    // my_pair.cmp_display(); // uncomment to see the error: method cannot be called on `Pair<Vec<{integer}>>` due to unsatisfied trait bounds

    // Blanket traits
    println!("-> Blanket Traits");
    //use crate::lib::MyBlanketTrait;  // bring 'MyBlanketTrait' into scope
    use lib::MyBlanketTrait;  // could be used since it has been brought into scope before

    println!("{}", 1568568.my_blanket_fn());
}


// Fixing the largest Function with Trait Bounds

// returns the largest element of a list (of any, but fixed, type), can be called only on types that implement the PartialOrd AND Copy traits!
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {  
    let mut largest_elem = list[0];

    for &item in list {
        if item > largest_elem {
            largest_elem = item
        }
    }

    largest_elem
}

fn largest_no_copy_trait<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_elem = &list[0];

    for item in list {
        if item > largest_elem {
            largest_elem = item
        }
    }

    largest_elem
}