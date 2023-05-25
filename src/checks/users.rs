use super::CheckType;
use regex::Regex;

use crate::checks::parser;

pub struct UsersCheck {
	pub check_name: String,
	pub names: Vec<String>,
	pub hosts: Vec<String>,
	pub terminals: Vec<String>,
}

impl CheckType for UsersCheck {
	fn run(&self) -> bool {
		
		let users = parser::users();

		// do a super inefficient search to check against the configured filters
		for user in users {

			for name in &self.names {

				let re = Regex::new(name).unwrap();
				if re.is_match(&user.user().unwrap()) {
					
					for host in &self.hosts {
						let re = Regex::new(host).unwrap();
						if re.is_match(&user.host().unwrap()) {
							
							for terminal in &self.terminals {
								let re = Regex::new(terminal).unwrap();
								if re.is_match(&user.terminal().unwrap()) {
									log::debug!("User {} is logged in from {} on {}", user.user().unwrap(), user.host().unwrap(), user.terminal().unwrap());
									return true;
								}
							}

						}
					}

				}

			}
		}

		return false;
	}

	fn get_check_name(&self) -> String {
		return self.check_name.clone();
	}
}