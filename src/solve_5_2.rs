use std::fs::File;
use std::io::Read;
use std::str::FromStr;

struct Program {
    pc: isize,
    ops: Vec<isize>
}

#[test]
fn given_example() {
    let mut prog = Program { pc: 0, ops: vec![0, 3, 0, 1, -3] };
    let answer = steps(&mut prog);
    assert_eq!(10, answer);
}

fn steps(prog: &mut Program) -> i32 {
    let mut counter = 0;
    while valid_pc(prog) {
        counter += 1;
        step(prog);
    }
    counter
}

fn step(prog: &mut Program) {
    let current_op = &mut prog.ops[prog.pc as usize];
    let new_pc = prog.pc + *current_op;
    if *current_op > 2 {
        *current_op -= 1;
    } else {
        *current_op += 1;
    }
    prog.pc = new_pc;
}

fn valid_pc(prog: &Program) -> bool {
    prog.pc >= 0 && prog.pc < prog.ops.len() as isize
}

pub fn solve(filename: &str) -> i32 {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let ops = contents.split('\n').map(|line| isize::from_str(line)).filter_map(|result| result.ok()).collect();
    let mut prog = Program { pc: 0, ops: ops };
    steps(&mut prog)
}
