use crate::node::SharedNode;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct SearchResult {
    node: SharedNode,
    hops: usize,
}

pub fn search_depth_first(node: &SharedNode, name: &str) -> Option<SearchResult> {
    let qm = QueueMeta {
        q: VecDeque::new(),
        dir: QueueDirection::Front,
    };
    return search_impl(node, name, qm);
}

pub fn search_breadth_first(node: &SharedNode, name: &str) -> Option<SearchResult> {
    let qm = QueueMeta {
        q: VecDeque::new(),
        dir: QueueDirection::Back,
    };
    return search_impl(node, name, qm);
}

#[derive(Debug)]
enum QueueDirection {
    Front,
    Back,
}

#[derive(Debug)]
struct QueueMeta {
    q: VecDeque<SharedNode>,
    dir: QueueDirection,
}

// internal handler for iterative tree traversal
// algorithm is identical except for the queue push direction
fn search_impl(node: &SharedNode, name: &str, mut qm: QueueMeta) -> Option<SearchResult> {
    let q = &mut qm.q;
    q.push_back(Rc::clone(node));

    let mut hops = 1;
    while !q.is_empty() {
        let node = q.pop_front().unwrap();

        if node.borrow().name == name {
            return Some(SearchResult {
                node: node.clone(),
                hops: hops,
            });
        }

        hops += 1;
        for child in &node.borrow().children {
            match qm.dir {
                QueueDirection::Front => q.push_front(child.clone()),
                QueueDirection::Back => q.push_back(child.clone()),
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::search_breadth_first;
    use super::search_depth_first;
    use crate::node::Node;
    use crate::node::SharedNode;
    use std::rc::Rc;

    fn assert_bfs(top: &SharedNode, name: &str, expected_node: &SharedNode, hops: usize) {
        let res = search_breadth_first(&top, name).unwrap();
        assert!(Rc::ptr_eq(&expected_node, &res.node));
        assert_eq!(res.node.borrow().name, expected_node.borrow().name);
        assert_eq!(res.hops, hops);
    }

    fn assert_dfs(top: &SharedNode, name: &str, expected_node: &SharedNode, hops: usize) {
        let res = search_depth_first(&top, name).unwrap();
        assert!(Rc::ptr_eq(&expected_node, &res.node));
        assert_eq!(res.node.borrow().name, expected_node.borrow().name);
        assert_eq!(res.hops, hops);
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

        assert_bfs(&top, "top", &top, 1);
        assert_bfs(&top, "c2", &c2, 3);
        assert_bfs(&top, "d3", &d3, 7);

        let res = search_breadth_first(&top, "not_exist");
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

        assert_dfs(&top, "top", &top, 1);
        assert_dfs(&top, "c2", &c2, 4);
        assert_dfs(&top, "d3", &d3, 3);

        let res = search_depth_first(&top, "not_exist");
        assert!(res.is_none());
    }
}
