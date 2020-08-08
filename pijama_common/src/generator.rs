pub struct Generator<T> {
    count: usize,
    f: fn(usize) -> T,
}

impl<T> Generator<T> {
    pub fn new(f: fn(usize) -> T) -> Self {
        Self { count: 0, f }
    }

    pub fn gen(&mut self) -> T {
        let item = (self.f)(self.count);
        self.count += 1;
        item
    }
}
