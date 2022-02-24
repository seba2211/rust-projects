#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// trait implementation - 1 
pub trait Summary {
    // fn summarize(&self) -> String;  // standard
    fn summarize(&self) -> String {  // provide a default implementation, it also calls another trait implementation
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;  // should be implemented by those using the trait, does not have a default implementation!
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters

/*
* The impl Trait syntax is convenient and makes for more concise code in simple cases.
*/
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
/*
* The trait bound syntax can express more complexity in other cases
*/
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax

/*
* pub fn notify(item: &(impl Summary + Display)) {}  
* the last is identical to
* pub fn notify<T: Summary + Display>(item: &T) {}
*/

// Clearer Trait Bounds with where Clauses

/*
* fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}  // this is hard to read!
* BUT, using 'where' clauses
* fn some_function<T, U>(t: &T, u: &U) -> i32
*    where T: Display + Clone,
*          U: Clone + Debug
* {}
* is more readable in complex cases!
*/

// Returning Types that Implement Traits

pub fn returns_summarizable() -> impl Summary {  // returns some type that implements the Summary trait without naming the concrete type
    Tweet {
        username: String::from("returns_summarizable"),
        content: String::from(
            "I was created inside a function that returns an 'impl Summary' !",
        ),
        reply: false,
        retweet: false,
    }
}

/*
* However, you can only use impl Trait in a function if you’re returning a single type!
*/

// Using Trait Bounds to Conditionally Implement Methods

use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {  // this implementation works for any type 'T'
    pub fn new(x: T, y: T) -> Self {  // notice the 'Self' syntax for constructing the object
        Self { x, y }
    }
}

impl<T: PartialOrd + Display> Pair<T> {  // this implementation works for types that implement 'Display' AND 'PartialOrd'
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        } 
    }
} 

/*
* We can also conditionally implement a trait for any type that implements another trait. 
* Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library
*/

pub trait MyBlanketTrait {
    fn my_blanket_fn(&self) -> String {
        format!("You could see this since this type implements the 'Display' trait!")
    }
}

impl<T> MyBlanketTrait for T
where T: Display,
{  // this trait is applied for all types that implement the 'Display' trait
}