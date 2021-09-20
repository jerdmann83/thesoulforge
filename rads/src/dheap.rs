#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    prio: i32,
    val: i32,
}

#[derive(Debug, PartialEq)]
struct DHeap {
    nodes: Vec<Node>,
}

const FACTOR: u32 = 3;

impl DHeap {
    pub fn new() -> Self {
        DHeap { nodes: vec![] }
    }

    pub fn insert(&mut self, n: Node) {
        self.nodes.push(n);
        self.bubble_up(self.nodes.len() - 1);
    }

    pub fn peek(&self) -> Node {
        self.nodes[0].clone()
    }

    pub fn peek_pos(&self, pos: usize) -> Option<Node> {
        if pos >= self.nodes.len() {
            return None;
        }
        Some(self.nodes[pos].clone())
    }

    pub fn pop(&mut self) -> Option<Node> {
        match self.nodes.len() {
            0 => None,
            1 => Some(self.nodes.pop().unwrap()),
            num => {
                self.nodes.swap(0, num - 1);
                let out = self.nodes.pop().unwrap();
                self.push_down(0);
                Some(out)
            }
        }
    }

    fn bubble_up(&mut self, start_idx: usize) {
        let mut idx = start_idx;
        while idx > 0 {
            let parent_idx = (idx - 1) / FACTOR as usize;
            if self.nodes[idx].prio > self.nodes[parent_idx].prio {
                self.nodes.swap(idx, parent_idx);
                idx = parent_idx
            } else {
                break;
            }
        }
    }

    fn push_down(&mut self, start_idx: usize) {
        let mut idx = start_idx;
        while idx < self.get_first_leaf_idx() {
            println!(
                "next:{} first:{} (len={})",
                idx,
                self.get_first_leaf_idx(),
                self.nodes.len()
            );
            let child_idx = self.get_max_child_idx(idx).unwrap();
            // if child_idx.is_none() {
            //     return;
            // }
            // let child_idx = child_idx.unwrap();

            assert!(child_idx > idx);
            let parent = &self.nodes[idx];
            let child = &self.nodes[child_idx];
            if child.prio > parent.prio {
                println!("swap {} and {}", idx, child_idx);
                self.nodes.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }
    }

    fn get_child_idxes(idx: usize) -> Vec<usize> {
        let mut out = vec![];
        for i in 0..FACTOR as usize {
            out.push(FACTOR as usize * idx + i + 1);
        }
        out
    }

    fn get_max_child_idx(&self, idx: usize) -> Option<usize> {
        let mut max_idx: Option<usize> = None;
        let mut max_prio = 0;
        for child_idx in Self::get_child_idxes(idx) {
            if child_idx >= self.nodes.len() {
                continue;
            }
            let child = &self.nodes[child_idx];
            if child.prio > max_prio {
                max_idx = Some(child_idx);
                max_prio = child.prio;
            }
        }
        // assert!(max_idx > idx);
        max_idx
    }

    fn max_nodes(levels: u32) -> usize {
        let mut out: usize = 0;
        for i in 0..levels {
            out += FACTOR.pow(i) as usize;
        }
        out
    }

    fn get_first_leaf_idx(&self) -> usize {
        match self.nodes.len() {
            0 => 0,
            _n => {
                let mut pow: u32 = 1;
                let mut last: usize = 0;

                while Self::max_nodes(pow) < self.nodes.len() {
                    pow += 1;
                }

                pow -= 1;
                let out: usize = Self::max_nodes(pow) + 1;
                assert!(out < self.nodes.len());
                out
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn indexes() {
        assert_eq!(DHeap::get_child_idxes(1), vec![4, 5, 6]);
        assert_eq!(DHeap::get_child_idxes(2), vec![7, 8, 9]);
        assert_eq!(DHeap::get_child_idxes(4), vec![13, 14, 15]);

        let mut d = DHeap::new();
        let lim = 5;
        for i in 1..lim {
            d.insert(Node { prio: i, val: i });
        }
        assert_eq!(d.get_first_leaf_idx(), 1);
        d.insert(Node {
            prio: lim + 1,
            val: lim + 1,
        });
        assert_eq!(d.get_first_leaf_idx(), 2);

        let mut d = DHeap::new();
        let lim = DHeap::max_nodes(3);
        for i in 1..lim {
            d.insert(Node {
                prio: i as i32 + 1,
                val: i as i32 + 1,
            });
        }
    }

    fn dheap() {
        let mut d = DHeap::new();
        let lim = 10;
        for i in 1..lim {
            d.insert(Node { prio: i, val: i });
        }

        assert_eq!(
            d.peek(),
            Node {
                prio: lim - 1,
                val: lim - 1
            }
        );

        for i in 1..lim {
            assert_eq!(d.pop(), Some(Node { prio: i, val: i }));
        }
    }

    #[test]
    fn push_down() {
        let mut rng = rand::thread_rng();
        let mut d = DHeap::new();
        for _ in 0..10 {
            let i: u16 = rng.gen();
            d.insert(Node {
                prio: i as i32,
                val: i as i32,
            });
        }

        let mut last = i32::MAX;
        while let Some(n) = d.pop() {
            println!("--- {} {}", last, n.prio);
            assert!(last >= n.prio);
            last = n.prio;
        }
    }

    #[test]
    fn max_child() {
        let mut d = DHeap::new();
        d.insert(Node { prio: 10, val: 10 });
        d.insert(Node { prio: 8, val: 8 });
        d.insert(Node { prio: 7, val: 7 });
        d.insert(Node { prio: 5, val: 5 });
        d.insert(Node { prio: 3, val: 3 });

        assert_eq!(d.get_max_child_idx(0), Some(1));
        assert_eq!(d.get_max_child_idx(1), Some(4));
    }
}
