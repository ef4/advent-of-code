#[derive(Debug,Clone)]
struct Hex (i32, i32, i32);

impl Hex {
    fn distance(&self, other : &Hex) -> i32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) / 2
    }
    fn direction(dir_name: &str) -> Hex {
        match dir_name {
            "n" => Hex(0, 1, -1),
            "s"=> Hex(0, -1, 1),
            "ne" => Hex(1, 0, -1),
            "sw" => Hex(-1, 0, 1),
            "se" => Hex(1, -1, 0),
            "nw" => Hex(-1, 1, 0),
            _ => panic!("unexpected direction {}", dir_name)
        }
    }
    fn add(&self, other : &Hex) -> Hex {
        Hex(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn solve(steps_str: &str) -> i32 {
    let origin = Hex(0,0,0);
    let steps = steps_str.split(',');
    let mut h = origin.clone();
    let mut max_distance = 0;
    for step in steps {
        let distance = h.distance(&origin);
        if distance > max_distance {
            max_distance = distance;
        }
        h = h.add(&Hex::direction(step.trim()));
    }
    max_distance
}

fn main() {
    println!("{}", solve(include_str!("../../inputs/input_11.txt")));
}



/*
  Local Variables:
    flycheck-rust-binary-name: "11-2"
  End:
*/
