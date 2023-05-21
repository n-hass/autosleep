pub trait CheckType {
  fn run(&self) -> bool;
  fn get_check_name(&self) -> String;
}

pub mod users;
pub mod command;
pub mod parser;