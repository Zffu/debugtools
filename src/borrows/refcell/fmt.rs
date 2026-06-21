use std::fmt::Display;

use crate::borrows::refcell::borrow::Borrow;

pub struct InvalidBorrowError {
    caused_by: Borrow,
    origins: Vec<Borrow>,
}

impl InvalidBorrowError {
    pub(crate) fn new(caused_by: Borrow, origins: Vec<Borrow>) -> Self {
        Self { caused_by, origins }
    }
}

impl Display for InvalidBorrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Invalid borrow error caused by:")?;
        writeln!(f, "{}", self.caused_by)?;

        writeln!(f, "Inherently caused by:")?;

        for origin in &self.origins {
            writeln!(f, "{}", origin)?;
        }

        Ok(())
    }
}

impl Display for Borrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} borrow at:",
            if self.is_mutable {
                "mutable"
            } else {
                "immutable"
            }
        )?;

        writeln!(f, "{}", self.position)
    }
}
