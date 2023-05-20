use super::CheckFn;
use std::process::Command;
use std::sync::{Arc, Mutex};
pub struct CommandCheck {
	pub command: Arc<Mutex<Command>>
}

impl CheckFn for CommandCheck {
	fn run(&self) -> bool {
		let mut command_local = self.command.lock().unwrap();
		match command_local.status() {
			Ok(output) => {
				return output.success();
			},
			Err(_) => {
				return false;
			}
		}
	}
}

