use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

#[cfg(test)]
static EXAMPLE : &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

#[test]
fn given_example() {
    let bottom = solve(EXAMPLE);
    assert_eq!("tknk", bottom);
}

#[test]
fn test_parser() {
    let circus = Circus::new(EXAMPLE);
    assert_eq!(72, *circus.weights.get("fwft").unwrap());
    assert_eq!("tknk", *circus.supported_by.get("fwft").unwrap());
}

pub struct Circus<'t> {
    pub weights: HashMap<&'t str, u32>,
    pub supported_by: HashMap<&'t str, &'t str>,
    pub supports: HashMap<&'t str, Vec<&'t str>>
}

impl<'t> Circus<'t> {
    pub fn new(input: &'t str) -> Circus<'t> {
        let mut circus = Circus { weights: HashMap::new(), supported_by: HashMap::new(), supports: HashMap::new() };
        let re = Regex::new(r"(\w+) \((\d+)\)(?: -> (.*))?").unwrap();
        for cap in re.captures_iter(input) {
            let name = &cap.get(1).unwrap().as_str();
            let weight = u32::from_str(&cap.get(2).unwrap().as_str()).unwrap();
            circus.weights.insert(*name, weight);
            if let &Some(m) = &cap.get(3) {
                let mut supports = vec![];
                for other in m.as_str().split(", ") {
                    circus.supported_by.insert(other, name);
                    supports.push(other);
                }
                circus.supports.insert(name, supports);
            }
        }
        circus
    }
}

pub fn bottom<'t>(circus: &Circus<'t>) -> &'t str {
    for name in circus.weights.keys() {
        if let Some(_) = circus.supported_by.get(*name) {

        } else {
            return name
        }
    }
    panic!("no bottomm")
}

pub fn solve<'t>(input: &'t str) -> &'t str {
    let circus = Circus::new(input);
    bottom(&circus)
}
