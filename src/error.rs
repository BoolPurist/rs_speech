use thiserror::Error;

pub const MIN: u32 = 1;
pub const MAX: u32 = 100;

#[derive(Debug)]
pub struct Precent(u32);

#[derive(Debug, Error)]
pub enum InvalidPrecent {
    #[error("Value {0} is higher than {MAX}")]
    TooHigh(u32),
    #[error("Value {0} is lower than {MIN}")]
    TooLow(u32),
}

impl InvalidPrecent {
    fn _value(&self) -> u32 {
        match self {
            Self::TooLow(number) | Self::TooHigh(number) => *number,
        }
    }
}

// impl Display for InvalidPrecent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             InvalidPrecent::TooLow(too_low) => {
//                 write!(f, "Value {} is smaller than {}", too_low, MIN)
//             }
//             InvalidPrecent::TooHigh(too_high) => {
//                 write!(f, "Value {} is smaller than {}", too_high, MAX)
//             }
//         }
//     }
// }

impl Precent {
    pub fn new(value: u32) -> Result<Self, InvalidPrecent> {
        if MIN > value {
            Err(InvalidPrecent::TooLow(value))
        } else if MAX < value {
            Err(InvalidPrecent::TooHigh(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn show(&self) -> String {
        let mut output = String::with_capacity(MAX as usize);

        let padding_amount = MAX - self.0;

        output.push_str(&"=".repeat(self.0 as usize));
        output.push_str(&" ".repeat(padding_amount as usize));
        output
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    #[test]
    fn should_return_precent() {
        let given = Precent::new(89);
        if let Ok(value) = given {
            assert_eq!(value.0, 89);
        } else {
            panic!("Expected ok with value 89")
        }
    }
    #[test]
    fn should_fail() {
        assert_case(101);
        assert_case(302);
        assert_case(0);

        fn assert_case(invalid: u32) {
            let given = Precent::new(invalid);
            if let Err(error) = given {
                assert_eq!(error._value(), invalid);
            }
        }
    }
}
