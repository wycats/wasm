use crate::prelude::*;

use codespan::{ByteIndex, ByteSpan};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Id(u32);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Spanned<Node: PartialEq + Debug> {
    crate node: Node,
    crate span: ByteSpan,
}

impl<Node: PartialEq + fmt::Display + fmt::Debug> fmt::Display for Spanned<Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}@{:?}", self.node, self.span)
        } else {
            write!(f, "{:?}", self.node)
        }
    }
}

crate trait SpannedItem: Sized + PartialEq + Debug {
    fn spanned(self, start: usize, l: usize, r: usize) -> Spanned<Self>;
    fn copy_span<T: PartialEq + Debug>(self, item: &Spanned<T>) -> Spanned<Self>;
    fn synthetic(self, desc: &'static str) -> Spanned<Self>;
}

impl<T: PartialEq + Debug> SpannedItem for T {
    fn spanned(self, start: usize, l: usize, r: usize) -> Spanned<T> {
        Spanned {
            node: self,
            span: ByteSpan::new(ByteIndex((start + l) as u32), ByteIndex((start + r) as u32)),
        }
    }

    fn copy_span<U: PartialEq + Debug>(self, item: &Spanned<U>) -> Spanned<Self> {
        Spanned {
            node: self,
            span: item.span,
        }
    }

    fn synthetic(self, _desc: &'static str) -> Spanned<T> {
        Spanned {
            node: self,
            span: ByteSpan::new(ByteIndex(0), ByteIndex(0)),
        }
    }
}
