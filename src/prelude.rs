//! Crate prelude

use std::rc::Rc;

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type BlockHash = [u8; 32];

