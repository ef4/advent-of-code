#[cfg(test)]
mod tests_1_2 {
    use solve_1_2::solve;

    #[test]
    fn first_example() {
        let answer = solve("1212");
        assert_eq!(6, answer);
    }

    #[test]
    fn second_example() {
        let answer = solve("1221");
        assert_eq!(0, answer);
    }

    #[test]
    fn third_example() {
        let answer = solve("123123");
        assert_eq!(12, answer);
    }

    #[test]
    fn fourth_example() {
        let answer = solve("12131415");
        assert_eq!(4, answer);
    }

}

pub fn solve(x: &str) -> u32 {
    let mut accum : u32 = 0;
    let digits: Vec<u32> = x.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let len = digits.len();
    for (index, digit) in digits.iter().enumerate() {
        if *digit == digits[(index + len/2) % len] {
            accum += *digit;
        }
    }
    accum
}




