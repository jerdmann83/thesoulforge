use std::cell::RefCell;
use std::rc::Rc;

type SharedNode = Rc<RefCell<Node>>;
type Nodes = Vec<SharedNode>;

mod search;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    children: Nodes,
}

#[derive(Debug)]
pub enum QueueLength {
    Size(usize),
    Null,
}

#[derive(Debug)]
pub struct SearchResult {
    node: SharedNode,
    hops: usize,
}

impl Node {
    fn new(name: &str) -> SharedNode {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            children: vec![],
        }))
    }

    pub fn get_depth(&self) -> usize {
        self.get_depth_impl()
    }

    fn get_depth_impl(&self) -> usize {
        let mut depths = vec![];
        // grab the depth of every child recursively
        for node in &self.children {
            depths.push(node.borrow().get_depth_impl());
        }
        let out = *depths.iter().max().unwrap_or(&(0 as usize));
        // depth at this level is the longest child plus the root node above it
        out + 1
    }

    fn push(&mut self, node: &SharedNode) {
        self.children.push(node.clone());
    }

    pub fn walk(&self) {
        self.walk_impl("", true);
    }

    fn walk_impl(&self, _indent: &str, _last: bool) {
        todo!();
        // println!("{}+- {}", indent, self.name);

        // let pipe = if last { " " } else { "|" } ;
        // let new_indent = format!("{}{}  ", indent, pipe);
        // for (idx, node) in self.children.iter().enumerate() {
        //     node.walk_impl(&new_indent, idx + 1 == node.children.len());
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::rc::Rc;

    #[test]
    fn get_depth_works() {
        let top_rc = Node::new("top");
        let mut top = top_rc.borrow_mut();
        assert_eq!(top.get_depth(), 1);

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.push(&c1);
        top.push(&c2);
        top.push(&c3);
        assert_eq!(top.get_depth(), 2);

        let long_top = Node::new("long");
        let add_count = 999;
        //  start to the top node
        let mut cur = long_top.clone();

        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in 0..add_count {
            // add some children
            for num in 0..rng.gen_range(2, 10) {
                cur.borrow_mut()
                    .push(&Node::new(&format!("c{}_{}", i, num)));
            }
            // // swing forward to the first newly-added child
            cur = cur.clone().borrow_mut().children[0].clone();
        }
        // total is the original root plus all levels we added
        assert_eq!(long_top.borrow().get_depth(), 1 + add_count);
    }
}
