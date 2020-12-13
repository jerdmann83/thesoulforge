use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

use petgraph::algo::has_path_connecting;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{depth_first_search, DfsEvent};

#[derive(Debug)]
struct Row {
    name: String,
    contents: Vec<String>,
}

fn parse(buf: &str) -> Vec<Row> {
    let mut out: Vec<Row> = vec![];
    for l in buf.split('\n') {
        // dark plum bags contain 4 drab aqua bags, 4 dull tomato bags.
        // bright turquoise bags contain no other bags.
        let toks: Vec<&str> = l.split("bags contain").collect();
        if toks.len() <= 1 {
            continue;
        }
        let name = toks[0].trim();
        if toks[1].contains("no other bags") {
            out.push(Row {
                name: name.to_string(),
                contents: vec![],
            });
            continue;
        }

        let mut contents = vec![];
        for t in toks[1].split(",") {
            let t = t.trim();
            let entry_toks: Vec<&str> = t.split_whitespace().collect();
            if entry_toks.len() < 3 {
                unreachable!();
            }
            let _ = entry_toks[0].parse::<usize>().unwrap();
            let name = entry_toks[1..3].join(" ");
            contents.push(name);
        }
        out.push(Row {
            name: name.to_string(),
            contents: contents,
        });
    }
    out
}
type GraphT = Graph<String, String>;
type IndexT = HashMap<String, NodeIndex>;

fn part1(rows: &Vec<Row>, goal_name: &str) -> usize {
    let mut g = Graph::<String, String>::new();
    let mut index: IndexT = HashMap::new();

    for row in rows {
        // reference to the current node.  either something added during a
        // previous iteration (eg a previous bag's child), or a new one that was
        // newly added to the graph
        let cur: NodeIndex;
        if let Some(exist) = index.get(&row.name) {
            cur = *exist;
        } else {
            let node = g.add_node(row.name.clone());
            index.insert(row.name.clone(), node);
            cur = node;
        }

        // add every child.  if the child already exists in the graph, point
        // this new parent at it
        for cname in &row.contents {
            let ccur: NodeIndex;
            if let Some(exist) = index.get(cname) {
                ccur = *exist;
            } else {
                let cnode = g.add_node(cname.clone());
                index.insert(cname.clone(), cnode);
                let cnode = index.get(cname).unwrap();
                ccur = *cnode;
            }
            g.extend_with_edges(&[(cur, ccur)]);
        }
    }

    let mut num = 0;
    let goal = index.get(goal_name).unwrap();
    for (_name, node) in &index {
        if *node != *goal {
            if has_path_connecting(&g, *node, *goal, None) {
                num += 1;
            }
        }
    }
    num
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let rows = parse(&buf);
    println!("{}", part1(&rows, "shiny gold"));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn iter() {
        let mut g = Graph::<&str, &str>::new();
        let root = g.add_node("root");
        let c1 = g.add_node("c1");
        let c2 = g.add_node("c2");
        let d1 = g.add_node("d1");
        let z = g.add_node("z");
        g.extend_with_edges(&[(root, c1), (root, d1), (c1, c2)]);

        // let out = dijkstra(&g, root, None, |_| 1);
        assert!(has_path_connecting(&g, root, c2, None));
        assert!(!has_path_connecting(&g, root, z, None));
    }

    #[test]
    fn test_part1() {
        let mut rows = vec![];
        rows.push(Row {
            name: "c1".to_string(),
            contents: vec![],
        });
        rows.push(Row {
            name: "b1".to_string(),
            contents: vec!["c1".to_string()],
        });
        rows.push(Row {
            name: "d1".to_string(),
            contents: vec!["b1".to_string()],
        });
        assert_eq!(part1(&rows, "c1"), 2);
    }
}
