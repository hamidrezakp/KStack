struct KStack<T> {
    array: Vec<T>,
    k: usize,
}

impl<T> KStack<T>
where
    T: Copy,
{
    pub fn new(k: usize) -> Self {
        KStack {
            array: Vec::<T>::new(),
            k: k,
        }
    }

    pub fn pop(self: &Self) -> Option<T> {
        self.array.pop()
    }

    pub fn push(self: &mut Self, item: T) {
        self.array.push(item);
    }

    pub fn kpop(self: &Self) -> Option<&[T]> {
        let array_len = self.array.len();
        &self
            .array
            .drain((array_len - self.k)..(array_len - 1))
            .collect()
    }

    pub fn kpush(self: &mut Self, items: &[T]) {
        self.array.extend_from_slice(items);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpop() {
        let stack = KStack::new(3);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(Some([1, 2, 3]), stack.kpop());
    }

    #[test]
    fn test_kpush() {
        let stack = KStack::new(3);
        stack.kpush(&[1, 2, 3]);

        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_push_pop() {
        let stack = KStack::new(3);
        stack.push(1);
        stack.push(2);

        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }
}
