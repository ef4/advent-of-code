use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashSet;
use regex::Regex;


type Banks = Vec<usize>;

#[test]
fn given_example() {
    let mut banks : Banks = vec![0, 2, 7, 0];
    let answer = steps(&mut banks);
    assert_eq!(5, answer);
}

#[test]
fn test_signature() {
    let banks : Banks = vec![0, 2, 7, 0];
    let answer = signature(&banks);
    assert_eq!("0,2,7,0", answer);
}


fn steps(banks: &mut Banks) -> u32 {
    let mut counter = 0;
    let mut seen_states = HashSet::<String>::new();
    let mut sig = signature(&banks);
    while !seen_states.contains(&sig) {
        seen_states.insert(sig);
        step(banks);
        sig = signature(&banks);
        counter += 1;
    }
    counter

}

fn step(banks: &mut Banks) {
    let mut max = None;
    for (index, block_count) in banks.iter().enumerate() {
        if let Some(max_index) = max {
            if banks[max_index] < *block_count {
                max = Some(index);
            }
        } else {
            max = Some(index);
        }
    }
    if let Some(mut index) = max {
        let mut remaining = banks[index];
        banks[index] = 0;
        while remaining > 0 {
            index = (index + 1) % banks.len();
            banks[index] += 1;
            remaining -= 1;
        }
    }
}

fn signature(banks : &Banks) -> String {
    banks.iter()
        .map(|block_count| format!("{}", block_count))
        .fold(String::new(), joiner)
}

fn joiner(mut accum : String, item: String) -> String {
    if !accum.is_empty() {
        accum.push(',');
    }
    accum.push_str(&item);
    accum
}


pub fn solve(filename: &str) -> u32 {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"\d+").unwrap();
    let mut banks: Banks = re.find_iter(&contents).filter_map(|m| usize::from_str(m.as_str()).ok()).collect();
    steps(&mut banks)
}
