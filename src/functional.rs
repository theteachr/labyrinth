use crate::Stack;

#[derive(Debug, Eq, PartialEq)]
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self::Empty
    }
}

impl<T> Stack<T> for List<T> {
    fn push(self, elem: T) -> Self {
        Self::Elem(elem, Box::new(self))
    }

    fn pop(self) -> Option<(T, Box<Self>)> {
        match self {
            Self::Empty => None,
            Self::Elem(i, list) => Some((i, list)),
        }
    }

    fn peek(&self) -> Option<&T> {
        match self {
            Self::Empty => None,
            Self::Elem(i, _) => Some(&i),
        }
    }
}
