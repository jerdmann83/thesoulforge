use std::cell::RefCell;
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

pub enum QueueLength {
    Size(usize),
    Null,
}

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

    fn push(&mut self, node: &SharedNode) {
        self.children.push(node.clone());
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

    pub fn search_breadth_first(&self, name: &str) -> Option<SearchResult> {
        println!("=== bfs ===");
        let mut q: VecDeque<SharedNode> = VecDeque::new();
        q.push_back(Rc::new(RefCell::new(self.clone())));

        let mut hops = 1;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            println!("check {} (want {})", &node.borrow().name, &name);
            if node.borrow().name == name {
                println!("bfs return {:?} clone", &node.as_ptr());
                return Some(SearchResult {
                    node: node.clone(),
                    hops: hops,
                });
            }

            hops += 1;
            for child in &node.borrow().children {
                q.push_back(child.clone());
            }
        }

        return None;
    }

    // todo: smash these into a common impl with two front-ends
    // the algorithms are identical other than the queue push direction
    pub fn search_depth_first(&self, name: &str) -> Option<SearchResult> {
        let mut q: VecDeque<SharedNode> = VecDeque::new();
        q.push_back(Rc::new(RefCell::new(self.clone())));

        let mut hops = 1;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            println!("check {} (want {})", &node.borrow().name, &name);
            if node.borrow().name == name {
                return Some(SearchResult {
                    node: node.clone(),
                    hops: hops,
                });
            }

            hops += 1;
            for child in &node.borrow().children {
                q.push_front(child.clone());
            }
        }
        return None;
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

    #[test]
    fn bfs_works() {
        let top = Node::new("top");

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.borrow_mut().push(&c1);
        top.borrow_mut().push(&c2);
        top.borrow_mut().push(&c3);

        let d1 = Node::new("d1");
        let d2 = Node::new("d2");
        let d3 = Node::new("d3");
        c1.borrow_mut().push(&d1);
        c2.borrow_mut().push(&d2);
        c3.borrow_mut().push(&d3);

        let res = top.borrow().search_breadth_first("top").unwrap();
        println!("{:?} {:?}", &top.as_ptr(), &res.node.as_ptr());
        assert!(Rc::ptr_eq(&top, &res.node));
        assert_eq!(res.node.borrow().name, top.borrow().name);
        assert_eq!(res.hops, 1);

        let res = top.borrow().search_breadth_first("c2").unwrap();
        assert_eq!(res.node.borrow().name, c2.borrow().name);
        assert_eq!(res.hops, 3);

        let res = top.borrow().search_breadth_first("d3").unwrap();
        assert_eq!(res.node.borrow().name, d3.borrow().name);
        assert_eq!(res.hops, 7);

        let res = top.borrow().search_breadth_first("not_exist");
        assert!(res.is_none());
    }

    #[test]
    fn dfs_works() {
        let top = Node::new("top");

        let c1 = Node::new("c1");
        let c2 = Node::new("c2");
        let c3 = Node::new("c3");
        top.borrow_mut().push(&c1);
        top.borrow_mut().push(&c2);
        top.borrow_mut().push(&c3);

        let d1 = Node::new("d1");
        let d2 = Node::new("d2");
        let d3 = Node::new("d3");
        c1.borrow_mut().push(&d1);
        c2.borrow_mut().push(&d2);
        c3.borrow_mut().push(&d3);

        let res = top.borrow().search_depth_first("top").unwrap();
        assert_eq!(res.node.borrow().name, top.borrow().name);
        assert_eq!(res.hops, 1);

        let res = top.borrow().search_depth_first("c2").unwrap();
        assert_eq!(res.node.borrow().name, c2.borrow().name);
        assert_eq!(res.hops, 4);

        let res = top.borrow().search_depth_first("d3").unwrap();
        assert_eq!(res.node.borrow().name, d3.borrow().name);
        assert_eq!(res.hops, 3);

        let res = top.borrow().search_depth_first("not_exist");
        assert!(res.is_none());
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
