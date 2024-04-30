//! Crate prelude

use std::{error::Error, rc::Rc};

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub type BlockHash = [u8; 32];

