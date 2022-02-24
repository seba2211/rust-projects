use super::*;

#[test]
fn yes_no_test1() {
    let str1 = "yes";
    assert!(yes_no(&str1), "str provided {}", str1);
}

#[test]
fn yes_no_test2() {
    let str1 = "no";
    assert!(!yes_no(&str1), "str provided {}", str1);
}

#[test]
fn yes_no_test3() {
    let str1 = " YeS    \n ";
    assert!(yes_no(&str1), "str provided {}", str1);
}

#[test]
fn yes_no_test4() {
    let str1 = " yesn't ";
    assert!(!yes_no(&str1), "str provided {}", str1);
}

#[test]
fn yes_no_test5() {
    let str1 = " Y    \n ";
    assert!(yes_no(&str1), "str provided {}", str1);
}

#[test]
fn yes_no_test6() {
    let str1 = " jskhfkjsd   \n ";
    assert!(!yes_no(&str1), "str provided {}", str1);
}