use crate::checks::*;
use command::CommandCheck;
use users::UsersCheck;
use log::{warn, error};

use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn check_enabled_field(field: Option<&Option<String>>) -> bool {
	let unwrapped = match field {
		Some(field) => field,
		None => {
			return true;
		}
	};
	match unwrapped {
    Some(enabled) => {
			match enabled.as_str() {
				"false" => {
					return false;
				},
				"true" => {return true;},
				_ => {
					warn!("Unknown value for enabled: \"{}\". Assuming is enabled", enabled);
					return true;
				}
			}
    },
    None => {return true;}
	};
}

fn create_command_check(config_section: &HashMap<String, Option<String>>, check_name: &str) -> Option<Box<CommandCheck>> {
	if check_enabled_field(config_section.get("enabled")) == false {
		return None;
	}
	
	let command_field = match config_section.get("command") {
		Some(command) => command,
		None => {
			error!("No command specified for Command check");
			return None;
		}
	};

	match command_field {
		Some(command) => {
			let command = Command::new(command);
			let check = CommandCheck {
				check_name: String::from(check_name),
				command: Arc::new(Mutex::new(command))
			};
			return Some(Box::new(check));
		},
		None => {
			error!("No command specified for check");
			return None;
		}
	}
}

fn create_users_check(config_section: &HashMap<String, Option<String>>, check_name: &str) -> Option<Box<UsersCheck>> {
	if check_enabled_field(config_section.get("enabled")) == false {
		return None;
	}

	let names_field = match config_section.get("names") {
		Some(names) => names,
		None => {
			error!("No names specified for Users check");
			return None;
		}
	};
	
	let names = match names_field {
		Some(names) => {
			let names: Vec<String> = names.split(',').map(|s| s.to_string()).collect();
			names
		},
		None => {
			error!("No names specified for check");
			return None;
		}
	};

	let hosts_field = match config_section.get("hosts") {
		Some(hosts) => hosts,
		None => {
			error!("No hosts specified for Users check");
			return None;
		}
	};
	let hosts = match hosts_field {
		Some(hosts) => {
			let hosts: Vec<String> = hosts.split(',').map(|s| s.to_string()).collect();
			hosts
		},
		None => {
			error!("No hosts specified for check");
			return None;
		}
	};

	let command = Command::new("who");

	return Some(Box::new(UsersCheck {
		check_name: String::from(check_name),
		names: names,
		hosts: hosts,
		check_command: Arc::new(Mutex::new(command))
	}));

}

pub fn create_checks( checks: &mut Vec<Box<dyn CheckType>>, config: &HashMap<String, HashMap<String, Option<String>>> ) {

	for section in config.keys() {		
		// if the section key begins with "check.", create a check struct and add it to the vector
		if section.starts_with("check.") {
			let check_name = section.trim_start_matches("check.");
			let check_section = config.get(section).unwrap();
			let check_class = check_section.get("class").unwrap();
			match check_class {
				Some(class) => {
					match class.as_str() {

						"Command" => {
							match create_command_check(check_section, check_name) {
								Some(check) => {
									checks.push(check);
								},
								None => {}
							}
						},
						"Users" => {
							match create_users_check(check_section, check_name) {
								Some(check) => {
									checks.push(check);
								},
								None => {}
							}
						},

						_ => {
							warn!("Unknown check class: {}", class);
						}

					}
				},
				None => {
					warn!("No check class specified for section {}", section);
				}
			}
		}

	}
}