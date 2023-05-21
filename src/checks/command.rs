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
		match command_local.output() {
			Ok(output) => {
				if output.status.success() {
					log::info!("Check \"{}\" reports activity", self.check_name);
					return true;
				}
				return false;
			},
			Err(_) => {
				return false;
			}
		}
	}

	fn get_check_name(&self) -> String {
		return self.check_name.clone();
	}
}

