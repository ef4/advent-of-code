#[cfg(test)]
mod tests_1_1 {
    use solve_1_1 as solve;

    #[test]
    fn first_example() {
        let answer = solve(1122);
        assert_eq!(3, answer);
    }

    #[test]
    fn second_example() {
        let answer = solve(1111);
        assert_eq!(4, answer);
    }

    #[test]
    fn third_example() {
        let answer = solve(1234);
        assert_eq!(0, answer);
    }

    #[test]
    fn fourth_example() {
        let answer = solve(91212129);
        assert_eq!(9, answer);
    }

}

use std::option::Option;

fn maybe_accum(accum: &mut i64, a: i64, b: i64) {
    if a == b {
        *accum += a;
    }
}

pub fn solve_1_1(x: i64) -> i64 {
    let mut remaining = x;
    let mut previous : Option<i64> = None;
    let mut accum : i64 = 0;
    while remaining > 0 {
        let digit = remaining % 10;
        if let Some(value) = previous {
            maybe_accum(&mut accum, value, digit);
        }
        remaining = remaining / 10;
        previous = Some(digit);
        if remaining == 0 {
            maybe_accum(&mut accum, digit, x % 10);
        }
    }

    accum
}
