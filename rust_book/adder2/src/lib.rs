
// 11.3 Test Organization

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal_function_test() {
        assert_eq!(internal_adder(2,2), 4)
    }
}