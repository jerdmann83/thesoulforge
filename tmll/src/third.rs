struct Buffer<T> {
    buf: Vec<T>,
}

impl<T> Buffer<T> {
    fn reset(&mut self) -> Vec<T> {
        std::mem::take(&mut self.buf)
    }
}
