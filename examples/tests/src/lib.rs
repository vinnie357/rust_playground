pub struct Guess {
    value: i32,
}

// default guess
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }
// uncomment me to fail the test
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }
// better error handling
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
// uncomment me to fail, i ad 3 instead of 2
// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
//uncomment me to fail, i don't send a name
// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// uncomment me to fail the test
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-results-with-the-assert-macro
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // default it works
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    // error handling itworks
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic]
    fn ifail() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    // default test
    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    // better test with message
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}