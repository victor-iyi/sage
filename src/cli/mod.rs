//! Command line interface module.
#![allow(dead_code)]

pub struct Cmd {}

impl Cmd {
  pub fn new() -> Result<Cmd, &'static str> {
    Ok(Cmd {})
  }
}