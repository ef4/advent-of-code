#[cfg(test)]
mod tests_2_1 {
    use solve_2_1::solve;

    #[test]
    fn example() {
        let answer = solve("5 1 9 5
                            7 5 3
                            2 4 6 8");
        assert_eq!(18, answer);
    }

    #[test]
    fn example2() {
        let answer = solve("7 5 30
                            2 4 6 8");
        assert_eq!(31, answer);
    }
}

use regex::Regex;
use std::str::FromStr;

pub fn solve(x: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut accum : u32 = 0;
    for line in x.lines() {
        let mut min : Option<u32> = None;
        let mut max : Option<u32> = None;
        for m in re.find_iter(line) {
            let digit = u32::from_str(m.as_str()).unwrap();
            if let Some(min_digit) = min {
                if min_digit > digit {
                    min = Some(digit);
                }
            } else {
                min = Some(digit);
            }
            if let Some(max_digit) = max {
                if max_digit < digit {
                    max = Some(digit);
                }
            } else {
                max = Some(digit);
            }
        }
        accum += max.unwrap() - min.unwrap();
    }
    accum
}
