#[cfg(test)]
mod tests_2_2 {
    use solve_2_2::solve;

    #[test]
    fn example() {
        let answer = solve("5 9 2 8
                            9 4 7 3
                            3 8 6 5");
        assert_eq!(9, answer);
    }

}

use regex::Regex;
use std::str::FromStr;

pub fn solve(x: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut accum : u32 = 0;
    for line in x.lines() {
        let nums : Vec<u32> = re
            .find_iter(line)
            .map(|m| u32::from_str(m.as_str()).unwrap())
            .collect();
        for (index, left) in nums.iter().enumerate() {
            for right in nums[index+1..].iter() {
                let min;
                let max;
                if left > right {
                    max = left;
                    min = right;
                } else {
                    max = right;
                    min = left;
                }
                if max % min == 0 {
                    accum += max / min;
                }
            }
        }

    }
    accum
}
