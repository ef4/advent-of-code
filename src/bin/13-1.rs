extern crate regex;
use regex::Regex;
use std::str::FromStr;

fn to_pair(captures: regex::Captures) -> (i32, i32) {
    let depth = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
    let range = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
    (depth, range)
}

fn hits(&(depth, range): &(i32, i32)) -> bool {
    depth % (2*(range - 1)) == 0
}

fn severity((depth, range): (i32, i32)) -> i32 {
    depth * range
}

#[test]
fn test_example() {
    let input = "0: 3
1: 2
4: 4
6: 4";
    assert_eq!(24, solve(input));

}


fn solve(puzzle_input: &str) -> i32 {
    let re = Regex::new(r"(\d+): (\d+)").unwrap();
    re.captures_iter(puzzle_input).map(to_pair).filter(hits).inspect(|x| println!("{:?}", x)).map(severity).fold(0, |a, b| a + b)
}

fn solve2(puzzle_input: &str) -> i32 {
    let re = Regex::new(r"(\d+): (\d+)").unwrap();
    let pairs : Vec<(i32, i32)> = re.captures_iter(puzzle_input).map(to_pair).collect();
    for delay in 0..100 {
        if (pairs.map(|(depth, range)| (depth + 1, range)).any(hits)) {
        } else {
            return delay
        }
    }
    panic!("No delay");
}


fn main() {
    let puzzle_input = include_str!("../../inputs/input_13.txt");
    println!("{} {}", solve(puzzle_input), solve2(puzzle_input));
}

/*
  Local Variables:
    flycheck-rust-binary-name: "13-1"
  End:
*/
