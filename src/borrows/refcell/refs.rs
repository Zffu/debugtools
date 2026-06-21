use std::{
    cell::{Ref, RefMut},
    ops::{Deref, DerefMut},
};

use crate::borrows::refcell::{Borrow, DebugRefCell};

/// A [`Ref`] given by a [`DebugRefCell`]
pub struct DebugRef<'a, T: Sized> {
    pub(crate) inner: Ref<'a, T>,
    pub(crate) borrow: Borrow,
    pub(crate) ref_cell: *const DebugRefCell<T>,
}

/// A [`RefMut`] given by a [`DebugRefCell`]
pub struct DebugRefMut<'a, T: Sized> {
    pub(crate) inner: RefMut<'a, T>,
    pub(crate) borrow: Borrow,
    pub(crate) ref_cell: *const DebugRefCell<T>,
}

impl<T> Drop for DebugRef<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.ref_cell
                .as_ref()
                .unwrap()
                .drop_ref(self.borrow.clone());
        }
    }
}

impl<T> Drop for DebugRefMut<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.ref_cell
                .as_ref()
                .unwrap()
                .drop_ref(self.borrow.clone());
        }
    }
}

impl<T> Deref for DebugRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for DebugRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for DebugRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
