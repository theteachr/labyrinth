pub mod functional;

trait Stack<T> {
    fn push(self, item: T) -> Self;
    fn pop(self) -> Option<(T, Box<Self>)>;
    fn peek(&self) -> Option<&T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use functional::List;

    #[test]
    fn push_and_peek_work() {
        assert_eq!(List::<u8>::default().push(4).peek(), Some(&4));
    }

    #[test]
    fn pop_works() {
        assert_eq!(List::<u8>::default().pop(), None);
        assert_eq!(
            List::default().push(4).push(3).pop(),
            Some((3, Box::new(List::Elem(4, Box::new(List::Empty)))))
        );
    }
}
