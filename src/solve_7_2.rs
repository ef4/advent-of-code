use solve_7_1::Circus;
use solve_7_1;
use std::iter::Iterator;
use std::collections::HashMap;
use std::hash::Hash;

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
    let answer = solve(EXAMPLE);
    assert_eq!(60, answer);
}

#[test]
fn total_weight_test() {
    let circus = Circus::new(EXAMPLE);
    let answer = total_weight(&circus, "tknk");
    assert_eq!(778, answer);
}

#[test]
fn test_outlier() {
    assert_eq!(None, outlier(vec![1,1,1].as_slice()));
    assert_eq!(Some(1), outlier(vec![1,2,1].as_slice()));
    assert_eq!(Some(2), outlier(vec![1,1,2].as_slice()));
    assert_eq!(None, outlier(vec!["1","1","1"].as_slice()));
    assert_eq!(Some(0), outlier(vec!["10","2","2"].as_slice()));
}


fn total_weight(circus: &Circus, name: &str) -> u32 {
    let own_weight = circus.weights.get(name).unwrap();
    if let Some(supported) = circus.supports.get(name) {
        supported.iter().fold(*own_weight, |acc, n| acc + total_weight(circus, n))
    } else {
        *own_weight
    }
}

fn outlier<T>(list: &[T]) -> Option<usize>
    where
    T : Eq, T: Hash
{
    if list.is_empty() {
        return None
    }

    let mut counts = HashMap::new();
    for elt in list {
        let next_count;
        if let Some(prev_count) = counts.get(&elt) {
            next_count = prev_count + 1;
        } else {
            next_count = 1;
        }
        counts.insert(elt, next_count);
    }

    let &min_count = counts.values().min().unwrap();
    let &max_count = counts.values().max().unwrap();
    if min_count != max_count {
        for (value, count) in counts {
            if count == min_count {
                return list.iter().position(|p| p.eq(value))
            }
        }
    }
    None
}


fn find_unbalanced<'t>(circus: &Circus<'t>, name: &'t str) -> Option<(&'t str, u32)> {
    match circus.supports.get(name) {
        Some(children) => {
            let weights : Vec<u32> = children.iter().map(|child| total_weight(circus, child)).collect();
            if let Some(unbalanced_index) = outlier(&weights) {
                let child = children.get(unbalanced_index).unwrap();
                let weight_diff = weights.get(unbalanced_index).unwrap() - weights.get((unbalanced_index + 1) % weights.len()).unwrap();
                let inner = find_unbalanced(circus, child);
                if inner.is_some() {
                    return inner
                } else {
                    return Some((child, circus.weights.get(child).unwrap() - weight_diff))
                }
            }
            None
        },
        None => None
    }
}


pub fn solve(input: &str) -> u32 {
    let circus = Circus::new(input);
    let bottom = solve_7_1::bottom(&circus);
    let (_, weight) = find_unbalanced(&circus, bottom).unwrap();
    weight
}
