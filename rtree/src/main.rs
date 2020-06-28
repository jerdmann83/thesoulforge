type Nodes = Vec<Box<Node>>;

pub struct Node {
    name: String,
    children: Nodes,
}

impl Node {
    fn new(name: &str) -> Node {
        Node{
            name: name.to_string(),
            children: vec!(),
        }
    }
}

pub fn walk(node: &Node) {
    walk_impl(&node, "", true);
}

fn walk_impl(node: &Node, indent: &str, last: bool) {
    println!("{}+- {}", indent, node.name);

    let pipe = if last { " " } else { "|" } ;
    let new_indent = format!("{}{}  ", indent, pipe);
    for (idx, node) in node.children.iter().enumerate() {
        walk_impl(node, &new_indent, idx + 1 == node.children.len());
    }
}

fn main() {
    let mut top = Node::new("top");

    let mut t1 = Node::new("t1");
    let mut g1 = Node::new("g1");
    let mut g2 = Node::new("g2");

    let mut t2 = Node::new("t2");
    let mut h1 = Node::new("h1");
    let mut h2 = Node::new("h2");

    let mut t3 = Node::new("t3");

    t1.children.push(Box::new(g1));
    t1.children.push(Box::new(g2));
    t2.children.push(Box::new(h1));
    t2.children.push(Box::new(h2));

    top.children.push(Box::new(t1));
    top.children.push(Box::new(t2));
    top.children.push(Box::new(t3));

    walk(&top);
}
