use std::cell::{RefCell, UnsafeCell};

use crate::borrows::refcell::{
    borrow::Borrow,
    fmt::InvalidBorrowError,
    refs::{DebugRef, DebugRefMut},
};

pub mod borrow;
pub mod fmt;
pub mod refs;

/// A [`RefCell`] that may be used for debugging purposes.
///
/// Just like a normal [`RefCell`], this implementation enforces the following rules:
/// - No active immutable borrows when a mutable borrow is created
/// - No multiple mutable borrows
///
/// However, unlike the normal implementation, this implementation gives more information over the borrowing such as for example which borrow is at fault
/// and what borrows ultimately caused this borrowing error.
///
/// # Example
/// ```
/// use debugtools::borrows::refcell::DebugRefCell;
///
///	let refcell: DebugRefCell<usize> = DebugRefCell::new(0);
///
/// *refcell.borrow_mut() = 5;
///
/// let val: usize = *refcell.borrow();
///
///
/// ```
///
pub struct DebugRefCell<T: Sized> {
    inner: RefCell<T>,

    mutable_borrows: UnsafeCell<Vec<Borrow>>,
    immutable_borrows: UnsafeCell<Vec<Borrow>>,
}

impl<T> DebugRefCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: RefCell::new(val),
            mutable_borrows: UnsafeCell::new(vec![]),
            immutable_borrows: UnsafeCell::new(vec![]),
        }
    }

    pub fn try_borrow(&self) -> Result<DebugRef<'_, T>, InvalidBorrowError> {
        let borrow = self.provide_borrow(false);

        self.check_borrowing_rules(borrow.clone(), false)?;

        unsafe { Ok(self.provide_unsafe_borrow(borrow)) }
    }

    pub fn try_borrow_mut(&self) -> Result<DebugRefMut<'_, T>, InvalidBorrowError> {
        let borrow = self.provide_borrow(true);

        self.check_borrowing_rules(borrow.clone(), true)?;

        unsafe { Ok(self.provide_unsafe_borrow_mut(borrow)) }
    }

    pub fn borrow(&self) -> DebugRef<'_, T> {
        match self.try_borrow() {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn borrow_mut(&self) -> DebugRefMut<'_, T> {
        match self.try_borrow_mut() {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }
}

impl<T> DebugRefCell<T> {
    fn check_borrowing_rules(
        &self,
        borrow: Borrow,
        mutable: bool,
    ) -> Result<(), InvalidBorrowError> {
        unsafe {
            if self.mutable_borrows.get().as_ref().unwrap().len() > 1 {
                return Err(InvalidBorrowError::new(
                    borrow,
                    self.mutable_borrows.get().as_ref().unwrap().clone(),
                ));
            }

            if mutable && !self.immutable_borrows.get().as_ref().unwrap().is_empty() {
                return Err(InvalidBorrowError::new(
                    borrow,
                    self.immutable_borrows.get().as_ref().unwrap().clone(),
                ));
            }

            Ok(())
        }
    }

    fn provide_borrow(&self, mutable: bool) -> Borrow {
        unsafe {
            let borrow = Borrow::new(
                mutable,
                if mutable {
                    self.mutable_borrows.get().as_ref().unwrap().len()
                } else {
                    self.immutable_borrows.get().as_ref().unwrap().len()
                },
            );

            if mutable {
                self.mutable_borrows
                    .get()
                    .as_mut()
                    .unwrap()
                    .push(borrow.clone());
            } else {
                self.immutable_borrows
                    .get()
                    .as_mut()
                    .unwrap()
                    .push(borrow.clone());
            }

            borrow
        }
    }

    unsafe fn provide_unsafe_borrow(&self, borrow: Borrow) -> DebugRef<'_, T> {
        DebugRef {
            inner: self.inner.borrow(),
            borrow: borrow,
            ref_cell: self as *const DebugRefCell<T>,
        }
    }

    unsafe fn provide_unsafe_borrow_mut(&self, borrow: Borrow) -> DebugRefMut<'_, T> {
        DebugRefMut {
            inner: self.inner.borrow_mut(),
            borrow: borrow,
            ref_cell: self as *const DebugRefCell<T>,
        }
    }

    pub(crate) fn drop_ref(&self, r: Borrow) {
        unsafe {
            if r.is_mutable {
                self.mutable_borrows.get().as_mut().unwrap().remove(r.index);
            } else {
                self.immutable_borrows
                    .get()
                    .as_mut()
                    .unwrap()
                    .remove(r.index);
            }
        }
    }
}
