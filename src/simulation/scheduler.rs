pub struct Scheduler {
    max_concurrent: usize,
}

impl Scheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }

    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }
}
