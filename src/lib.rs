#[cfg(test)]
mod tests_1_1 {
    use solve_1_1::solve_1_1 as solve;

    #[test]
    fn first_example() {
        let answer = solve("1122");
        assert_eq!(3, answer);
    }

    #[test]
    fn second_example() {
        let answer = solve("1111");
        assert_eq!(4, answer);
    }

    #[test]
    fn third_example() {
        let answer = solve("1234");
        assert_eq!(0, answer);
    }

    #[test]
    fn fourth_example() {
        let answer = solve("91212129");
        assert_eq!(9, answer);
    }

}

pub mod solve_1_1 {
    use std::option::Option;

    pub fn solve_1_1(x: &str) -> u32 {
        let mut previous : Option<char> = None;
        let mut accum : u32 = 0;
        let mut first : Option<char> = None;
        for digit in x.chars() {
            if let Some(value) = previous {
                if value == digit {
                    accum += digit.to_digit(10).unwrap();
                }
            } else {
                first = Some(digit);
            }
            previous = Some(digit);
        }

        if let Some(first_value) = first {
            if let Some(previous_value) = previous {
                if first_value == previous_value {
                    accum += previous_value.to_digit(10).unwrap();
                }
            }
        }

        accum
    }
}


