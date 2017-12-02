#[cfg(test)]
mod tests_1_1 {
    use solve_1_1::solve;

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

pub fn solve(x: &str) -> u32 {
    let mut accum : u32 = 0;
    let digits: Vec<u32> = x.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let len = digits.len();
    for (index, digit) in digits.iter().enumerate() {
        if *digit == digits[(index + 1) % len] {
            accum += *digit;
        }
    }
    accum
}
