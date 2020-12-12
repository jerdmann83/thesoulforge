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
        // let mut cur = g.add_node(name.to_string());

        // nodes.push(cur);
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

// fn add(name: &str, g: &mut GraphT, i: &mut IndexT) -> NodeIndex {
//     let ni = g.add_node(name.to_string);
// }

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut index: IndexT = HashMap::new();
    let mut pending: IndexT = HashMap::new();

    let rows = parse(&buf);
    let mut g = Graph::<String, String>::new();
    // let root = g.add_node("root");
    // let d1 = g.add_node("d1");
    // g.extend_with_edges(&[(root, c1), (root, d1), (c1, c2)]);

    for row in rows {
        if let Some(exist) = index.get(&row.name) {
            // g.extend_with_edges(&[(node, *exist)]);
        }

        let node = g.add_node(row.name.clone());
        index.insert(row.name.clone(), node);
        for child in row.contents {
            if let Some(exist) = index.get(&child) {
                g.extend_with_edges(&[(node, *exist)]);
                println!("extend {}->{}", &row.name, &child);
            // let ni = g.add_node(name.to_string);
            // println!("{:?}", exist);
            } else {
                let child_node = g.add_node(child.clone());
                index.insert(child.clone(), child_node);
                g.extend_with_edges(&[(node, child_node)]);
                println!("new {}->{}", &row.name, &child);
            }
        }
    }

    let mut num = 0;
    let goal = index.get("shiny gold").unwrap();
    for (name, node) in &index {
        if has_path_connecting(&g, *node, *goal, None) {
            num += 1;
        }
    }
    println!("{}", num);
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
}
