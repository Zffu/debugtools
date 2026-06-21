use std::backtrace::Backtrace;

/// Represents a borrow happening inside of a debug refcell
#[derive(Clone)]
pub(crate) struct Borrow {
    pub index: usize,
    pub is_mutable: bool,
    pub position: String,
}

impl Borrow {
    pub(crate) fn new(is_mutable: bool, index: usize) -> Self {
        Self {
            index,
            is_mutable,
            position: format!("{}", Backtrace::capture()),
        }
    }
}
