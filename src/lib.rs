pub mod functional;

trait Stack {
    fn push(self, item: i32) -> Self;
    fn pop(self) -> Option<(i32, Box<Self>)>;
    fn peek(&self) -> Option<i32>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use functional::List;

    #[test]
    fn push_and_peek_work() {
        assert_eq!(List::new().push(4).peek(), Some(4));
    }

    #[test]
    fn pop_works() {
        assert_eq!(List::new().pop(), None);
        assert_eq!(
            List::new().push(4).push(3).pop(),
            Some((3, Box::new(List::Elem(4, Box::new(List::Empty)))))
        );
    }
}
