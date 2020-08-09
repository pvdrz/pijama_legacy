pub(super) struct Generator<T> {
    count: usize,
    f: fn(usize) -> T,
}

impl<T> Generator<T> {
    pub(super) fn new(f: fn(usize) -> T) -> Self {
        Self { count: 0, f }
    }

    pub(super) fn gen(&mut self) -> T {
        let item = (self.f)(self.count);
        self.count += 1;
        item
    }
}
