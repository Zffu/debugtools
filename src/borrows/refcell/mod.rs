use std::cell::RefCell;

use crate::borrows::refcell::borrow::Borrow;

pub mod borrow;
pub mod refs;

pub struct DebugRefCell<T: Sized> {
    inner: RefCell<T>,

    mutable_borrows: Vec<Borrow>,
    immutable_borrows: Vec<Borrow>,
}

impl<T> DebugRefCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: RefCell::new(val),
            mutable_borrows: vec![],
            immutable_borrows: vec![],
        }
    }

    pub(crate) fn drop_ref(&mut self, r: Borrow) {
        if r.is_mutable {
            self.mutable_borrows.remove(r.index);
        } else {
            self.immutable_borrows.remove(r.index);
        }
    }
}
