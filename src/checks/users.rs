use super::CheckType;
use std::process::Command;
use std::str;
use std::sync::{Arc, Mutex};
use regex::Regex;

pub struct UsersCheck {
	pub check_name: String,
	pub names: Vec<String>,
	pub hosts: Vec<String>,
	pub check_command: Arc<Mutex<Command>>
}

impl CheckType for UsersCheck {
	fn run(&self) -> bool {
		let mut check_command_local = self.check_command.lock().unwrap();


		match check_command_local.output() {
			Ok(output) => {
				let output_str = str::from_utf8(&output.stdout).unwrap();
				let lines: Vec<&str> = output_str.split('\n').collect();
				
				// use regex to search for each name in the line
				let mut found = false;
				for line in lines {
					for name in &self.names {
						let re_name = Regex::new(&format!("{}", name)).unwrap();
						if re_name.is_match(line) {
							for host in &self.hosts {
								let re_host = Regex::new(&format!("{}", host)).unwrap();
								if re_host.is_match(line) {
									found = true;
									break;
								}
							}
						}
					}
					if found {
						break;
					}
				}
			},
			Err(_) => {
				return false;
			}
		}

		return true;
		
	}

	fn get_check_name(&self) -> String {
		return self.check_name.clone();
	}
}

