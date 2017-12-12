use std::str::FromStr;

#[derive(Debug)]
struct Knot {
    current_index: usize,
    points: Vec<i32>,
    skip_size: usize,
    size: usize
}

impl Knot {
    fn new(n: usize) -> Knot {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            vec.push(i as i32);
        }
        Knot { points: vec, current_index: 0, skip_size: 0, size: n }
    }
    fn twist(&mut self, k: i32) {
        let mut head = self.current_index;
        let mut tail = (self.current_index + k as usize - 1) % self.size;
        for _ in 0..k/2 {
            let tmp = self.points[tail];
            self.points[tail] = self.points[head];
            self.points[head] = tmp;
            head = (head + 1) % self.size;
            tail = positive_rem(tail as i32 - 1 , self.size);
        }

        self.current_index = (self.current_index + k as usize + self.skip_size ) % self.size;
        self.skip_size += 1;
    }
}

fn positive_rem(a: i32, b: usize) -> usize {
    let c = a % (b as i32);
    if c < 0 {
        (c + b as i32) as usize
    } else {
        c as usize
    }
}

#[test]
fn test_example() {
    let mut k = Knot::new(5);
    for &twist in [3, 4, 1, 5].iter() {
        k.twist(twist);
    }
    assert_eq!(vec![3,4,2,1,0], k.points);
}

fn main() {
    let mut k = Knot::new(256);
    let twists = "14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244".split(",").map(|n| i32::from_str(n).unwrap());
    for twist in twists {
        k.twist(twist);
    }
    println!("{}", k.points[0] * k.points[1]);
}

/*
  Local Variables:
    flycheck-rust-binary-name: "10-1"
  End:
*/
