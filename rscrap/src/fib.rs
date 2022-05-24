pub fn fib_recursive(n: u64, depth: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    return fib_recursive(n - 1, depth + 1) + fib_recursive(n - 2, depth + 1);
}

pub fn fib_dynamic(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut vals = vec![0, 1];
    for i in 2..n as usize {
        vals.push(vals[i - 1] + vals[i - 2]);
    }
    *vals.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fib() {
        println!("{}", fib_recursive(20, 0));
        println!("{}", fib_dynamic(20));
    }
}
