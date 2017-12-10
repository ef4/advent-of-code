use std::str::Chars;
static FULL_INPUT : &str = include_str!("../inputs/input_9.txt");

#[test]
fn test_examples() {
    assert_eq!(1, process("{}").0);
    assert_eq!(6, process("{{{}}}").0);
    assert_eq!(5, process("{{},{}}").0);
    assert_eq!(16, process("{{{},{},{{}}}}").0);
    assert_eq!(1, process("{<a>,<a>,<a>,<a>}").0);
    assert_eq!(9, process("{{<ab>},{<ab>},{<ab>},{<ab>}}").0);
    assert_eq!(9, process("{{<!!>},{<!!>},{<!!>},{<!!>}}").0);
    assert_eq!(3, process("{{<a!>},{<a!>},{<a!>},{<ab>}}").0);

}

fn process(input: &str) -> (i32, i32) {
    let mut stream = input.chars();
    let mut depth = 0;
    let mut score = 0;
    let mut garbage = 0;
    outer_mode(&mut stream, &mut depth, &mut score, &mut garbage);
    (score, garbage)
}

pub fn solve() -> (i32, i32) {
    process(FULL_INPUT)
}

fn outer_mode(stream : &mut Chars, depth: &mut i32, score: &mut i32, garbage: &mut i32) {
    while let Some(char) = stream.next() {
        match char {
            '{' => {
                *depth += 1;
            },
            '}' => {
                *score += *depth;
                *depth -= 1;
            },
            ',' => (),
            '<' => inner_mode(stream, garbage),
            '\n' => (),
            _ => panic!("Unsupported char in outer_mode {}", char)

        }
    }
}

fn inner_mode(stream: &mut Chars, garbage: &mut i32) {
    let mut cancel_next = false;
    while let Some(char) = stream.next() {
        if cancel_next {
            cancel_next = false;
        } else {
            match char {
                '>' => {
                    break;
                },
                '!' => {
                    cancel_next = true;
                },
                _ => {
                    *garbage += 1;
                }
            }
        }
    }

}
