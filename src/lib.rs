#[derive(Debug, Eq, PartialEq)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

impl List {
    pub fn push(self, elem: i32) -> Self {
        Self::Elem(elem, Box::new(self))
    }

    pub fn pop(self) -> Option<(i32, Box<Self>)> {
        match self {
            Self::Empty => None,
            Self::Elem(i, list) => Some((i, list)),
        }
    }

    pub fn peek(&self) -> Option<i32> {
        match self {
            Self::Empty => None,
            Self::Elem(i, _) => Some(*i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_peek_work() {
        assert_eq!(List::Empty.push(4).peek(), Some(4));
    }

    #[test]
    fn pop_works() {
        assert_eq!(List::Empty.pop(), None);
        assert_eq!(
            List::Empty.push(4).push(3).pop(),
            Some((3, Box::new(List::Elem(4, Box::new(List::Empty)))))
        );
    }
}
