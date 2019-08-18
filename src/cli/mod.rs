//! Command line interface module.
#![allow(dead_code)]

pub struct Argument {}

impl Argument {
  pub fn new() -> Result<Argument, &'static str> {
    Ok(Argument {})
  }
}