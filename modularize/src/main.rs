// Rust -> project structure -> Based on: https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee

mod something;  // bring the module into scope
use something::a::A;  // use an element from the module

fn main() {
    let first = A { a: 42, };
    let second = something::b::B {b: 64};

    println!("f = {}, s = {}", first.a, second.b);

}