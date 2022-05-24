struct VecIter<'a, T> {
    pos: usize,
    vec: &'a Vec<T>,
}

impl<'a, T> VecIter<'a, T> {
    fn new(vec: &'a Vec<T>) -> Self {
        VecIter { pos: 0, vec }
    }
}

impl<T: Clone> Iterator for VecIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.vec.len() {
            return None;
        }
        let out = &self.vec[self.pos];
        self.pos += 1;
        Some(out.clone())
    }
}

#[cfg(test)]
mod test {
    use super::VecIter;

    #[test]
    fn vec_iter() {
        let v = vec![1, 2, 3];
        let mut i1 = VecIter::new(&v);
        let mut i2 = VecIter::new(&v);
        let mut i3 = VecIter::new(&v);
        assert_eq!(i1.next(), Some(1));
        assert_eq!(i1.next(), Some(2));
        assert_eq!(i1.next(), Some(3));
        assert_eq!(i2.next(), Some(1));
        assert_eq!(i3.next(), Some(1));

        assert_eq!(i1.next(), None);
        assert_eq!(i1.next(), None);
        assert_eq!(i2.next(), Some(2));
        assert_eq!(i2.next(), Some(3));
    }
}
