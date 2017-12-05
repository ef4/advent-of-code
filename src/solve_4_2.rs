use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[test]
fn valid_passes() {
    let answer = valid_passphrase("abcde fghij");
    assert_eq!(true, answer);
}

#[test]
fn invalid_fails() {
    let answer = valid_passphrase("abcde xyz ecdab");
    assert_eq!(false, answer);
}

fn valid_passphrase(phrase : &str) -> bool {
    if phrase.is_empty() {
        return false
    }
    let mut words = HashSet::new();
    for word in phrase.split(' ') {
        let mut bytes = word.to_owned().into_bytes();
        bytes.sort();
        let sorted_word = String::from_utf8(bytes).unwrap();

        if words.contains(&sorted_word) {
            return false
        } else {
            words.insert(sorted_word.clone());
        }
    }
    true
}

pub fn solve(filename: &str) -> i32 {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split('\n').map(|line| valid_passphrase(line)).fold(0, |accum, valid| if valid { accum + 1 } else { accum })
}
