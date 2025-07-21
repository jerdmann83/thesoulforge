use std::collections::HashMap;
fn main() {
    let mut h : HashMap<i32, i32> = HashMap::new();
    h.insert(1, 2);
    h.entry(1).and_modify(|e| *e = 99);
    println!("{:?}", h[&1]);
    println!("{:?}", h);
}
