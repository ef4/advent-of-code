//static FULL_INPUT : &str = include_str!("../inputs/input_8.txt");
static SIMPLE_INPUT: &str = include_str!("../inputs/input_8s.txt");
use regex::Regex;
use std::str::FromStr;

#[test]
fn test_simple_parse() {
    let prog = Program::new(SIMPLE_INPUT);
    assert_eq!(4, prog.commands.len());
    assert_eq!("b", prog.commands[0].register);
    assert_eq!("inc", prog.commands[0].operation);
    assert_eq!(5, prog.commands[0].arg);
    assert_eq!(-10, prog.commands[2].arg);
    assert_eq!("c", prog.commands[3].predicate_register);
    assert_eq!("==", prog.commands[3].predicate_operation);
    assert_eq!(10, prog.commands[3].predicate_arg);

}

struct Program<'t> {
    commands: Vec<Command<'t>>
}

impl<'t> Program<'t> {
    fn new(source_code: &'t str) -> Program<'t> {
        Program { commands: source_code.lines().filter_map(Command::new).collect() }
    }
}

struct Command<'t> {
    register: &'t str,
    operation: &'t str,
    arg: i32,
    predicate_register: &'t str,
    predicate_operation: &'t str,
    predicate_arg: i32
}


impl<'t> Command<'t> {
    fn new(source_line: &'t str) -> Option<Command<'t>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) (\w+) (-?\d+) if (\w+) (.*) (-?\d+)").unwrap();
        }
        RE.captures(source_line)
            .map(|captures|
                 Command {
                     register: captures.get(1).unwrap().as_str(),
                     operation: captures.get(2).unwrap().as_str(),
                     arg: i32::from_str(captures.get(3).unwrap().as_str()).unwrap(),
                     predicate_register: captures.get(4).unwrap().as_str(),
                     predicate_operation: captures.get(5).unwrap().as_str(),
                     predicate_arg: i32::from_str(captures.get(6).unwrap().as_str()).unwrap()
                 }
            )
    }
}

pub fn solve() -> i32 {
    0
}
