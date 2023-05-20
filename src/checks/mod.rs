pub trait CheckFn {
  fn run(&self) -> bool;
}

pub mod users;
pub mod command;

// pub enum Checks {
// 	Users(users::UsersCheck),
// 	Script(command::CommandCheck),
// }