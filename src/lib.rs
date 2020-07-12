#![feature(const_generics)]

/// Stack with a windows view of K top items.
/// its like an normal stack and have `push` and `pop`
/// methods, also have three other method. `kpush`, `kpop` and
/// `kshow`.
///
/// The K defines window size, so you can see, push or pop K
/// top element on top of stack.
/// /*!
///     |  4  |   <--+
///     |  3  |      + K = 3
///     |  2  |   <--+
///     |  1  |
///     =======
/// */
///
pub struct KStack<T, const K: usize>(Vec<T>);

impl<T, const K: usize> KStack<T, K>
where
    T: Copy,
{
    /// Make a new KStack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    /// ```
    ///
    pub fn new() -> Self {
        KStack(Vec::<T>::new())
    }

    /// Remove and get single element on top of stack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    ///
    ///     stack.push(1);
    ///     stack.push(2);
    ///
    ///    assert_eq!(Some(2), stack.pop());
    ///    assert_eq!(Some(1), stack.pop());
    ///    assert_eq!(None, stack.pop());
    /// ```
    ///
    pub fn pop(self: &mut Self) -> Option<T> {
        self.0.pop()
    }

    /// Push a single element to stack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    ///
    ///     stack.push(1);
    ///     stack.push(2);
    ///
    ///    assert_eq!(Some(2), stack.pop());
    ///    assert_eq!(Some(1), stack.pop());
    ///    assert_eq!(None, stack.pop());
    /// ```
    ///
    pub fn push(self: &mut Self, item: T) {
        self.0.push(item);
    }

    /// Removes and return K top element on stack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    ///
    ///     stack.push(1);
    ///     stack.push(2);
    ///     stack.push(3);
    ///     stack.push(4);
    ///
    ///    assert_eq!([Some(4), Some(3), Some(2)], stack.kpop());
    ///    assert_eq!([Some(1), None, None], stack.kpop());
    /// ```
    ///
    pub fn kpop(self: &mut Self) -> [Option<T>; K] {
        let mut result: [Option<T>; K] = [None; K];
        for i in 0..=(K - 1) {
            result[i] = self.0.pop();
        }
        result
    }

    /// return K top element on stack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    ///
    ///     stack.push(1);
    ///     stack.push(2);
    ///     stack.push(3);
    ///     stack.push(4);
    ///
    ///    assert_eq!([Some(4), Some(3), Some(2)], stack.kshow());
    ///    assert_eq!([Some(4), Some(3), Some(2)], stack.kpop());
    /// ```
    ///
    pub fn kshow(self: &mut Self) -> [Option<T>; K] {
        let mut result: [Option<T>; K] = [None; K];

        match self.0.len() {
            0 => result,
            x if x < K => {
                for i in 0..self.0.len() {
                    result[i] = Some(self.0[self.0.len() - i - 1]);
                }
                result
            }
            _ => {
                for i in 0..=(K - 1) {
                    result[i] = Some(self.0[self.0.len() - i - 1]);
                }
                result
            }
        }
    }

    /// Push K element to stack.
    ///
    /// # Example
    /// ```
    ///     use kstack::KStack;
    ///
    ///     let mut stack = KStack::<i32, 3>::new();
    ///
    ///     stack.kpush(&[4, 5, 6, 7]);
    ///
    ///     assert_eq!(Some(7), stack.pop());
    ///     assert_eq!(Some(6), stack.pop());
    ///     assert_eq!(Some(5), stack.pop());
    ///     assert_eq!(Some(4), stack.pop());
    ///     assert_eq!(None, stack.pop());
    /// ```
    ///
    pub fn kpush(self: &mut Self, items: &[T]) {
        self.0.extend_from_slice(items);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpop() {
        let mut stack = KStack::<i32, 3>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!([Some(4), Some(3), Some(2)], stack.kpop());
        assert_eq!([Some(1), None, None], stack.kpop());
    }

    #[test]
    fn test_kshow() {
        let mut stack = KStack::<i32, 3>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!([Some(4), Some(3), Some(2)], stack.kshow());
        assert_eq!([Some(4), Some(3), Some(2)], stack.kshow());
    }

    #[test]
    fn test_kshow_underflow() {
        let mut stack = KStack::<i32, 3>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!([Some(4), Some(3), Some(2)], stack.kshow());
        assert_eq!([Some(4), Some(3), Some(2)], stack.kpop());
        assert_eq!([Some(1), None, None], stack.kshow());
        assert_eq!([Some(1), None, None], stack.kpop());
        assert_eq!([None, None, None], stack.kshow());
    }

    #[test]
    fn test_kpush() {
        let mut stack = KStack::<i32, 3>::new();
        stack.kpush(&[4, 5, 6, 7]);

        assert_eq!(Some(7), stack.pop());
        assert_eq!(Some(6), stack.pop());
        assert_eq!(Some(5), stack.pop());
        assert_eq!(Some(4), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_push_pop() {
        let mut stack = KStack::<i32, 3>::new();
        stack.push(1);
        stack.push(2);

        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }
}
