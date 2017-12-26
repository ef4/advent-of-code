extern crate regex;


#[test]
fn test_example() {
    let answer = solve(5, 65, 8921);
    assert_eq!(1, answer);

}

fn solve(steps: i64, start_a: i64, start_b: i64) -> i64 {
    let mut a = start_a;
    let mut b = start_b;
    let mut counter = 0;
    for _ in 0..steps {
        loop {
            a = (a * 16807) % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (b * 48271) % 2147483647;
            if b % 8 == 0 {
                break;
            }
        };
        if a & 0xffff == b & 0xffff {
            counter += 1;
        }
    }
    counter
}


fn main() {
    println!("{}", solve(5000000, 679, 771));
}

/*
  Local Variables:
    flycheck-rust-binary-name: "15"
  End:
*/
