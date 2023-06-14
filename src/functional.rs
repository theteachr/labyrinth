use crate::Stack;

#[derive(Debug, Eq, PartialEq)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

impl List {
    pub fn new() -> Self {
        Self::Empty
    }
}

impl Stack for List {
    fn push(self, elem: i32) -> Self {
        Self::Elem(elem, Box::new(self))
    }

    fn pop(self) -> Option<(i32, Box<Self>)> {
        match self {
            Self::Empty => None,
            Self::Elem(i, list) => Some((i, list)),
        }
    }

    fn peek(&self) -> Option<i32> {
        match self {
            Self::Empty => None,
            Self::Elem(i, _) => Some(*i),
        }
    }
}
