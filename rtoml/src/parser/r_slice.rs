use crate::lexer::TokenSized;
use crate::parser::r_iter::RIter;

use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Default, Debug)]
pub struct RIndex {
    pub line: usize,
    pub col: usize,
}

impl RIndex {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RSlice<'a> {
    pub slice: &'a [TokenSized<'a>],
    pub index: RIndex,
    pub peeked: Option<&'a TokenSized<'a>>,
}

impl<'a> From<&RIter<'a>> for RSlice<'a> {
    fn from(iter: &RIter<'a>) -> Self {
        Self {
            slice: iter.iter.as_slice(),
            index: iter.index,
            peeked: iter.peeked,
        }
    }
}

impl Display for RIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, column: {}", self.line, self.col)
    }
}
