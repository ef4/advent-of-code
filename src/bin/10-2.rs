#[derive(Debug)]
struct Knot {
    current_index: usize,
    points: Vec<u8>,
    skip_size: usize,
    size: usize
}

impl Knot {
    fn new(n: usize) -> Knot {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            vec.push(i as u8);
        }
        Knot { points: vec, current_index: 0, skip_size: 0, size: n }
    }
    fn twist(&mut self, k: u8) {
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

fn parse_and_pad(input: &str) -> Vec<u8> {
    let mut bytes = input.bytes().collect::<Vec<_>>();
    for suffix in [17, 31, 73, 47, 23].iter() {
        bytes.push(*suffix);
    }
    bytes
}

#[test]
fn test_parse() {
    let input = "1,2,3";
    assert_eq!(vec![49,44,50,44,51,17,31,73,47,23], parse_and_pad(input));
}

fn knot_hash(input : &str) -> String {
    let bytes = parse_and_pad(input);
    let mut knot = Knot::new(256);
    for _ in 0..64 {
        for twist in bytes.iter() {
            knot.twist(*twist);
        }
    }
    let mut counter = 0;
    let mut accum : u8 = 0;
    let mut output = String::with_capacity(32);
    for point in knot.points {
        accum = accum ^ point;
        counter += 1;
        if counter == 16 {
            output.push_str(&format!("{:02x}", accum));
            counter = 0;
            accum = 0;
        }
    }
    output
}

#[test]
fn test_hashes() {
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash("AoC 2017"));
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash(""));
}



fn main() {
    let input = "14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244";
    println!("{}", knot_hash(input));
}

/*
  Local Variables:
    flycheck-rust-binary-name: "10-2"
  End:
*/
