use super::CheckType;
use std::process::Command;
use std::sync::{Arc, Mutex};
pub struct CommandCheck {
	pub check_name: String,
	pub command: Arc<Mutex<Command>>
}

impl CheckType for CommandCheck {
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

	fn getCheckName(&self) -> String {
		return self.check_name.clone();
	}
}

