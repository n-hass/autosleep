#![allow(non_snake_case)]
pub trait CheckType {
  fn run(&self) -> bool;
  fn getCheckName(&self) -> String;
}

pub mod users;
pub mod command;