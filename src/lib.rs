//! Debugtools is a library used in Rust to debug annoying problems such as RefCell borrow errors and more with more ease.
//! Please note that these tools are mostly made to be used for debugging problems and guarantee no additional safety over
//! their normal counterparts and may only be used for debugging purposes.
//!
//! These implementations try to be as close as their implemented counterpart

pub mod borrows;
