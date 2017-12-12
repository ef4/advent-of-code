extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Graph<'t> {
    edges: HashMap<&'t str, Vec<&'t str>>
}

impl<'t> Graph<'t> {
    fn new(puzzle_input: &'t str) ->  Graph<'t> {
        let re = Regex::new(r"(\d+) <-> ((?:\d+(?:, )?)+)").unwrap();
        let mut graph = Graph{ edges: HashMap::new() };
        for captures in re.captures_iter(puzzle_input) {
            let id = captures.get(1).unwrap().as_str();
            let links : Vec<&str> = captures.get(2).unwrap().as_str()
                .split(", ").collect();
            graph.edges.insert(id, links);

        }
        graph
    }
    fn walk(&'t self, start_node: &'t str) -> GraphWalker<'t> {
        GraphWalker { graph: self, queue: vec![start_node], seen: [start_node].iter().cloned().collect() }
    }
}

struct GraphWalker<'t> {
    graph: &'t Graph<'t>,
    queue: Vec<&'t str>,
    seen: HashSet<&'t str>
}

impl<'t> Iterator for GraphWalker<'t> {
    type Item = &'t str;
    fn next(&mut self) -> Option<&'t str> {
        match self.queue.pop() {
            Some(node) => {
                if let Some(links) = self.graph.edges.get(node) {
                    for link in links {
                        if !self.seen.contains(link) {
                            self.seen.insert(link);
                            self.queue.push(link);
                        }
                    }
                }
                Some(node)
            },
            None => None
        }
    }
}

fn solve<'t>(puzzle_input: &'t str) -> i32 {
    let g = Graph::new(puzzle_input);
    g.walk("0").fold(0, |a,_| a + 1)
}

fn main() {
    println!("{:?}", solve(include_str!("../../inputs/input_12.txt")));
}



/*
  Local Variables:
    flycheck-rust-binary-name: "12-1"
  End:
*/
