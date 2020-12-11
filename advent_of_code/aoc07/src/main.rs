use std::io::stdin;
use std::io::Read;

#[derive(Debug)]
pub struct BagLink {
    num: usize,
    bag: Box<Bag>,
}

#[derive(Debug)]
pub struct Bag {
    color: String,
    bags: Vec<BagLink>,
}

#[derive(Debug)]
pub struct BagIter<'a> {
    stack: Vec<&'a Bag>,
}

impl<'a> BagIter<'a> {
    pub fn new(bag: &'a Bag) -> Self {
        let bit = BagIter { stack: vec![&bag] };
        bit
    }
}

impl<'a> Iterator for BagIter<'a> {
    type Item = &'a Bag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }
        let out = self.stack.pop().unwrap();
        for b in &out.bags {
            self.stack.push(&b.bag);
        }
        Some(out)
    }
}

impl Bag {
    pub fn new(color: &str) -> Box<Self> {
        let b = Bag {
            color: color.to_string(),
            bags: vec![],
        };
        Box::new(b)
    }

    pub fn add(&mut self, bag: Box<Bag>, num: usize) {
        self.bags.push(BagLink { bag: bag, num: num });
    }

    pub fn contains(&self, color: &str) -> bool {
        let mut bag_it = BagIter::new(&self);
        while let Some(b) = bag_it.next() {
            if b.color == color {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut all_bags: Vec<Box<Bag>> = vec![];

    for l in buf.split('\n') {
        // dark plum bags contain 4 drab aqua bags, 4 dull tomato bags.
        // bright turquoise bags contain no other bags.
        let toks: Vec<&str> = l.split("bags contain").collect();
        if toks.len() <= 1 {
            continue;
        }
        let name = toks[0].trim();
        let mut cur = Bag::new(name);
        if toks[1].contains("no other bags") {
            all_bags.push(cur);
            continue;
        }

        for t in toks[1].split(",") {
            let t = t.trim();
            let entry_toks: Vec<&str> = t.split_whitespace().collect();
            if entry_toks.len() < 3 {
                unreachable!();
            }
            let num = entry_toks[0].parse::<usize>().unwrap();
            let name = entry_toks[1..3].join(" ");
            cur.bags.push(BagLink {
                num: num,
                bag: Bag::new(&name),
            });
        }
        all_bags.push(cur);
    }

    let mut num = 0;
    for b in &all_bags {
        if b.contains("shiny gold") {
            num += 1
        }
    }
    println!("{}", num);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn iter() {
        let mut root = Bag::new("root");
        root.add(Bag::new("blue"), 2);
        root.add(Bag::new("red"), 1);

        let mut bag_it = BagIter::new(&root);
        let mut colors_in = vec!["root", "blue", "red"];
        let mut colors_out = vec![];
        while let Some(b) = bag_it.next() {
            colors_out.push(&b.color);
        }
        colors_in.sort();
        colors_out.sort();
        assert_eq!(colors_in, colors_out);

        assert!(root.contains("blue"));
        assert!(!root.contains("yellow"));
    }

    #[test]
    fn depth() {
        let mut root = Bag::new("root");
        let mut cur: &mut Box<Bag> = &mut root;
        let depth = 100;
        for i in 1..depth {
            cur.add(Bag::new(&format!("bag{}", i)), i);
            cur = &mut cur.bags[0].bag;
        }

        let mut i = 0;
        let mut bag_it = BagIter::new(&root);
        while let Some(_) = bag_it.next() {
            i += 1;
        }
        assert_eq!(i, depth);
        assert!(root.contains(&format!("bag{}", depth - 1)));
        assert!(root.contains(&format!("bag{}", depth - 2)));
    }
}
