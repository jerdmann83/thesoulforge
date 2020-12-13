use std::collections::HashMap;
use std::fmt;
use std::io::stdin;
use std::io::Read;

use petgraph::algo::has_path_connecting;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::depth_first_search;
use petgraph::visit::{Control, DfsEvent};

#[derive(Debug)]
struct BagMeta {
    name: String,
    num: u32,
}

#[derive(Debug)]
struct Row {
    name: String,
    contents: Vec<BagMeta>,
}

type GraphT = Graph<String, u32>;
type IndexT = HashMap<String, NodeIndex>;

struct GraphMeta {
    graph: GraphT,
    index: IndexT,
}

impl GraphMeta {
    fn new(rows: &Vec<Row>) -> Self {
        let mut g = GraphT::new();
        let mut index: IndexT = HashMap::new();

        for row in rows {
            // reference to the current node.  either something added during a
            // previous iteration (eg a previous bag's child), or a new one that
            // was newly added to the graph
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
            for cmeta in &row.contents {
                let ccur: NodeIndex;
                if let Some(exist) = index.get(&cmeta.name) {
                    ccur = *exist;
                } else {
                    let cnode = g.add_node(cmeta.name.clone());
                    index.insert(cmeta.name.clone(), cnode);
                    let cnode = index.get(&cmeta.name).unwrap();
                    ccur = *cnode;
                }
                g.add_edge(cur, ccur, cmeta.num);
            }
        }

        GraphMeta {
            graph: g,
            index: index,
        }
    }
}

// graph lib serializer is ours too because why not?
impl fmt::Display for GraphMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
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
            let num = entry_toks[0].parse::<u32>().unwrap();
            let name = entry_toks[1..3].join(" ");
            contents.push(BagMeta {
                name: name,
                num: num,
            });
        }
        out.push(Row {
            name: name.to_string(),
            contents: contents,
        });
    }
    out
}

fn part1(rows: &Vec<Row>, goal_name: &str) -> usize {
    let gm = GraphMeta::new(&rows);

    let mut num = 0;
    let goal = gm.index.get(goal_name).unwrap();
    for (_name, node) in &gm.index {
        if *node != *goal {
            if has_path_connecting(&gm.graph, *node, *goal, None) {
                num += 1;
            }
        }
    }
    num
}

fn part2(rows: &Vec<Row>, start_name: &str) -> usize {
    let gm = GraphMeta::new(&rows);
    println!("{}", gm);

    let mut weight = 1;
    let start = gm.index.get(start_name).unwrap();

    depth_first_search(&gm.graph, Some(*start), |event| {
        println!("{:?} ", event);
        if let DfsEvent::Discover(node, _time) = event {
            for e in gm.graph.edges_directed(node, petgraph::Direction::Outgoing) {
                // println!("---- edge {:?} {:?}", node, e.weight());
                weight *= *e.weight() as usize;
            }
        }
    });
    weight
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let rows = parse(&buf);
    println!("{}", part1(&rows, "shiny gold"));
    println!("{}", part2(&rows, "shiny gold"));
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
            contents: vec![BagMeta {
                name: "c1".to_string(),
                num: 1,
            }],
        });
        rows.push(Row {
            name: "d1".to_string(),
            contents: vec![BagMeta {
                name: "b1".to_string(),
                num: 1,
            }],
        });
        assert_eq!(part1(&rows, "c1"), 2);
    }

    #[test]
    fn test_part2() {
        let mut rows = vec![];
        rows.push(Row {
            name: "c1".to_string(),
            contents: vec![],
        });
        rows.push(Row {
            name: "b1".to_string(),
            contents: vec![BagMeta {
                name: "c1".to_string(),
                num: 3,
            }],
        });
        rows.push(Row {
            name: "d1".to_string(),
            contents: vec![BagMeta {
                name: "b1".to_string(),
                num: 2,
            }],
        });

        // expecting 9 bags
        // amazing ascii diagram:
        //         d1
        //      /     \
        //    b1       b1
        //  / | \    / | \
        // c1 c1 c1 c1 c1 c1

        assert_eq!(part2(&rows, "d1"), 9);
    }
}
