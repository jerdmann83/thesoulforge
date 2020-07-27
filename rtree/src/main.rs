use std::cell::{RefCell};
// use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

type SharedNode = Rc<RefCell<Node>>;
type Nodes = Vec<SharedNode>;

#[derive(Clone)]
pub struct Node {
    name: String,
    children: Nodes,
}

pub struct SearchResult {
    node: Option<SharedNode>,
    qlen: usize,
}

impl Node {
    fn new(name: &str) -> SharedNode {
        Rc::new(RefCell::new(Node{
            name: name.to_string(),
            children: vec!(),
        }))
    }

    fn push(&mut self, node: SharedNode) {
        self.children.push(node);
    }

    pub fn get_depth(&self) -> usize {
        self.get_depth_impl()
    }

    fn get_depth_impl(&self) -> usize {
        let mut depths = vec!();
        // grab the depth of every child recursively
        for node in &self.children {
            depths.push(node.borrow().get_depth_impl());
        }
        let out = *depths.iter().max().unwrap_or(&(0 as usize));
        // depth at this level is the longest child plus the root node above it
        out + 1
    }

    pub fn search_breadth_first(&self, name: &str) -> SearchResult {
        let q = RefCell::new(VecDeque::new());
        // queue up the node rc's
        self.search_breadth_first_impl(name, &q);

        // walk them to see if we got a match
        let mut idx = 0;
        for node in q.borrow().iter() {
            if node.name == name {
                return SearchResult{
                    node: Some(Rc::new(RefCell::new(node.clone()))),
                    qlen: idx+1};
            }

            idx += 1;
        }
        return SearchResult{node: None, qlen: q.borrow().len()};
    }

    fn search_breadth_first_impl(&self, name: &str, q: &RefCell<VecDeque<Node>>) {
        // qlen += 1
        q.borrow_mut().push_back(self.clone());
        for node in &self.children {
            node.borrow().search_breadth_first_impl(name, &q);
        }
    }

    pub fn walk(&self) {
        self.walk_impl("", true);
    }

    fn walk_impl(&self, indent: &str, last: bool) {
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
mod tests{
    use std::cell::{RefCell};
    use std::rc::Rc;
    use super::Node;
    use super::SearchResult;

    #[test]
    fn get_depth_works() {
        let top_rc = Node::new("top");
        let mut top = top_rc.borrow_mut();
        assert_eq!(top.get_depth(), 1);

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.push(c1);
        top.push(c2);
        top.push(c3);
        assert_eq!(top.get_depth(), 2);

        let long_top = Node::new("long");
        let add_count = 999;
        //  start to the top node
        let mut cur = long_top.clone();
        for i in 0..add_count {
            // add a child
            println!("{}", cur.borrow_mut().name);
            cur.borrow_mut().push(Node::new(&format!("c{}", i)));
            // // swing forward to the newly-added child
            cur = cur.clone().borrow_mut().children[0].clone();
        }
        // total is the original root plus all nodes we added
        assert_eq!(long_top.borrow().get_depth(), 1 + add_count);
    }

    #[test]
    fn bfs_works() {
        let mut top = Node::new("top");

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.borrow_mut().push(c1);
        top.borrow_mut().push(c2);
        top.borrow_mut().push(c3);

        let nil = Node::new("_nil");
        // assert_eq!(ptr::eq(
        //     top.search_breadth_first("top").node.as_ref().unwrap(), &top), true);

        let res = top.borrow().search_breadth_first("top");
        assert_eq!(res.node.unwrap_or(nil.clone()).borrow().name, top.borrow().name);
        assert_eq!(res.qlen, 1);

        let res = top.borrow().search_breadth_first("c3");
        assert_eq!(res.node.unwrap_or(nil.clone()).borrow().name, top.borrow().name);
        assert_eq!(res.qlen, 1);

        let res = top.borrow().search_breadth_first("not_exist");
        assert!(res.node.unwrap_or(nil.clone()).borrow().name == nil.borrow().name);
        assert_eq!(res.qlen, 1);

            // .unwrap_or(SearchResult{
            // node: Some(Node::new(nil)),
            // qlen: 0);
            // Node::new("top").name);
        // assert_eq!(top.search_breadth_first("nonexist").unwrap_or(&Node::new(nil)).name,
        //     Node::new(nil).name);
    }
}

fn main() {
    // let mut top = Node::new("top");

    // let mut t1 = Node::new("t1");
    // let g1 = Node::new("g1");
    // let g2 = Node::new("g2");

    // let mut t2 = Node::new("t2");
    // let h1 = Node::new("h1");
    // let h2 = Node::new("h2");

    // let t3 = Node::new("t3");

    // t1.push(g1);
    // t1.push(g2);
    // t2.push(h1);
    // t2.push(h2);

    // top.push(t1);
    // top.push(t2);
    // top.push(t3);

    // top.walk();
}
