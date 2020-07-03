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

    fn push(&mut self, node: Node) {
        self.children.push(Box::new(node));
    }

    pub fn get_depth(&self) -> usize {
        self.get_depth_impl()
    }

    fn get_depth_impl(&self) -> usize {
        let mut depths = vec!();
        // grab the depth of every child recursively
        for node in &self.children {
            depths.push(node.get_depth_impl());
        }
        let out = *depths.iter().max().unwrap_or(&(0 as usize));
        // depth at this level is the longest child plus the root node above it
        out + 1
    }

    pub fn walk(&self) {
        self.walk_impl("", true);
    }

    fn walk_impl(&self, indent: &str, last: bool) {
        println!("{}+- {}", indent, self.name);

        let pipe = if last { " " } else { "|" } ;
        let new_indent = format!("{}{}  ", indent, pipe);
        for (idx, node) in self.children.iter().enumerate() {
            node.walk_impl(&new_indent, idx + 1 == node.children.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    #[test]
    fn get_depth_works() {
        let mut top = Node::new("top");
        assert_eq!(top.get_depth(), 1);

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.push(c1);
        top.push(c2);
        top.push(c3);
        assert_eq!(top.get_depth(), 2);

        let mut long_top = Node::new("long");
        let add_count = 999;
        // refer to the top node
        let mut cur = &mut long_top;
        for i in 0..add_count {
            // add a child and swing forward to it
            cur.push(Node::new(&format!("c{}", i)));
            cur = &mut *cur.children[0];
        }
        // total is the original root plus all nodes we added
        assert_eq!(long_top.get_depth(), 1 + add_count);
    }
}

fn main() {
    let mut top = Node::new("top");

    let mut t1 = Node::new("t1");
    let g1 = Node::new("g1");
    let g2 = Node::new("g2");

    let mut t2 = Node::new("t2");
    let h1 = Node::new("h1");
    let h2 = Node::new("h2");

    let t3 = Node::new("t3");

    t1.push(g1);
    t1.push(g2);
    t2.push(h1);
    t2.push(h2);

    top.push(t1);
    top.push(t2);
    top.push(t3);

    top.walk();
}
