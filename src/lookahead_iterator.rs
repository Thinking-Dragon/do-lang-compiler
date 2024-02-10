pub struct LookAheadIterator<T: Clone> {
    data: Vec<T>,
    index: usize,
}

impl<T: Clone> LookAheadIterator<T> {
    pub fn new(data: Vec<T>) -> Self {
        LookAheadIterator {
            data: data,
            index: 0,
        }
    }

    pub fn lookahead(&mut self, offset: usize) -> Option<&T> {
        match self.index + offset < self.data.len() {
            true  => Some(&self.data[self.index + offset]),
            false => None,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        match self.index < self.data.len() {
            true => {
                self.index += 1;
                Some(self.data[self.index - 1].clone())
            },
            false => None,
        }
    }
}
