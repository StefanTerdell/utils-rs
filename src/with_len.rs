pub trait WithLen {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn not_empty(&self) -> bool {
        !self.is_empty()
    }
}
