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

fn handle_line(line: &str) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    let nums : Vec<u32> = RE
        .find_iter(line)
        .map(|m| u32::from_str(m.as_str()).unwrap())
        .collect();

    for (index, left) in nums.iter().enumerate() {
        for right in nums[index+1..].iter() {
            let min : &u32;
            let max : &u32;
            if left > right {
                max = left;
                min = right;
            } else {
                max = right;
                min = left;
            }
            if max % min == 0 {
                return max / min;
            }
        }
    }
    0
}

pub fn solve(x: &str) -> u32 {
    x.lines().map(handle_line).fold(0, |a, q| a+q)
}
