use crate::lexer::{Token, TokenSized};

use std::iter::Iterator;
use std::slice::Iter;

use crate::parser::r_slice::{RIndex, RSlice};

#[derive(Clone, Debug)]
pub struct RIter<'a> {
    pub iter: Iter<'a, TokenSized<'a>>,
    pub index: RIndex,
    peeked: Option<&'a TokenSized<'a>>,
}

impl<'a> RIter<'a> {
    pub fn new(slice: &'a [TokenSized<'a>]) -> Self {
        Self {
            iter: slice.iter(),
            index: RIndex::new(),
            peeked: None,
        }
    }
    pub fn as_slice(&self) -> RSlice<'a> {
        RSlice::from(self)
    }

    pub fn peek(&mut self) -> Option<&TokenSized> {
        if self.peeked.is_some() {
            self.peeked
        } else {
            self.peeked = self.next();
            self.peeked
        }
    }

    pub fn next_if_eq(&mut self, token: &Token) -> bool {
        if let Some((x, _)) = self.peek() {
            if x == token {
                self.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a> From<RSlice<'a>> for RIter<'a> {
    fn from(slice: RSlice<'a>) -> Self {
        Self {
            iter: slice.slice.iter(),
            index: RIndex::new(),
            peeked: None,
        }
    }
}

impl<'a> Iterator for RIter<'a> {
    type Item = &'a TokenSized<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.peeked {
            self.peeked = None;
            Some(x)
        } else {
            let x = self.iter.next();
            match x {
                Some((Token::LineBreak, _)) => {
                    self.index.col = 0;
                    self.index.line += 1;
                }
                Some((_, n)) => self.index.col += n,
                _ => (),
            }
            x
        }
    }
}
