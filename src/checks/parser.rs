use std::process::Command;
use std::str;

pub struct User {
	user: String,
	tty: String,
	host: String
}

impl User {
	pub fn user(&self) -> Option<&str> {
		Some(&*self.user)
	}

	pub fn terminal(&self) -> Option<&str> {
		Some(&*self.tty)
	}

	pub fn host(&self) -> Option<&str> {
		Some(&*self.host)
	}
}

pub fn users() -> Vec<User> {
	let mut w_command = Command::new("w");
	let captured_output: String = match w_command.arg("--from").arg("--short").arg("--no-header").output() {
		Ok(output) => {
			let output_str = str::from_utf8(&output.stdout).unwrap();
			output_str.to_string()
		},
		Err(_) => {
			String::from("")
		}
	};

	let captured_lines: Vec<&str> = captured_output.split('\n').collect();

	let mut users: Vec<User> = Vec::new();

	for line in captured_lines {
		let cols: Vec<&str> = line.split_whitespace().collect();
		if cols.len()<3 {continue}
		let (l_name, l_tty, l_host) = (cols[0], cols[1], cols[2]);

		let user = User {
			user: String::from(l_name),
			tty: String::from(l_tty),
			host: String::from(l_host)
		};

		users.push(user);
	}

	return users;
}