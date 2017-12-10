static FULL_INPUT : &str = include_str!("../inputs/input_8.txt");

#[cfg(test)]
static SIMPLE_INPUT: &str = include_str!("../inputs/input_8s.txt");

use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

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

#[test]
fn test_command_run() {
    let prog = Program::new(SIMPLE_INPUT);
    let mut machine = Machine::new();
    prog.commands[1].run(&mut machine);
    assert_eq!(1, *machine.registers.get("a").unwrap());
    assert_eq!(1, machine.largest_value());
}

#[test]
fn test_program_run() {
    let prog = Program::new(SIMPLE_INPUT);
    let mut machine = Machine::new();
    machine.execute(&prog);
    assert_eq!(1, machine.largest_value());
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

    fn run(&self, machine : &mut Machine) {
        let predicate_value = machine.read(self.predicate_register);
        let should_run = match self.predicate_operation {
            "<" => predicate_value < self.predicate_arg,
            ">" => predicate_value > self.predicate_arg,
            ">=" => predicate_value >= self.predicate_arg,
            "<=" => predicate_value <= self.predicate_arg,
            "==" => predicate_value == self.predicate_arg,
            "!=" => predicate_value != self.predicate_arg,
            _ => panic!("Unsupported predicate operation {}", self.predicate_operation)
        };
        if should_run {
            machine.increment(self.register, match self.operation {
                "dec" => -1 * self.arg,
                "inc" => self.arg,
                _ => panic!("Unsupported operation {}", self.operation)
            });
        }
    }
}

struct Machine {
    registers: HashMap<String, i32>,
    max_ever: i32
}

impl Machine {
    fn new() -> Machine {
        Machine { registers: HashMap::new(), max_ever: 0 }
    }
    fn execute<'t>(&mut self, program : &Program<'t>) {
        for command in program.commands.iter() {
            command.run(self);
        }
    }
    fn read(&self, register : &str) -> i32 {
        match self.registers.get(register) {
            Some(&value) => value,
            None => 0
        }
    }
    fn increment(&mut self, register: &str, amount: i32) {
        let value = self.read(register) + amount;
        self.registers.insert(register.to_owned(), value);
        if value > self.max_ever {
            self.max_ever = value;
        }
    }
    fn largest_value(&self) -> i32 {
        match self.registers.values().max() {
            Some(&value) => value,
            None => 0
        }
    }
}

pub fn solve() -> (i32,i32) {
    let prog = Program::new(FULL_INPUT);
    let mut machine = Machine::new();
    machine.execute(&prog);
    (machine.largest_value(), machine.max_ever)
}
