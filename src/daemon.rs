use crate::checks::CheckFn;
use std::collections::HashMap;
use std::process::Command;

use log::{info, warn, error};

pub async fn d_loop(checks: &Vec<Box<dyn CheckFn>>, config: &HashMap<String, HashMap<String, Option<String>>>) {
	let suspend_cmd = match config.get("general").unwrap().get("suspend_cmd").unwrap() {
		Some(cmd) => cmd,
		None => {
			error!("No suspend command specified");
			return;
		}
	};
	let mut suspend_cmd = Command::new(suspend_cmd);
	let interval = match config.get("general").unwrap().get("interval").unwrap() {
		Some(interval) => {
			match interval.parse::<u32>() {
				Ok(interval) => interval,
				Err(_) => {
					error!("Invalid interval specified");
					return;
				}
			}
		},
		None => {
			error!("No interval specified");
			return;
		}
	};
	let idle_limit = match config.get("general").unwrap().get("idle_time").unwrap() {
		Some(idle_time) => {
			match idle_time.parse::<u32>() {
				Ok(idle_time) => idle_time,
				Err(_) => {
					error!("Invalid idle time specified");
					return;
				}
			}
		},
		None => {
			error!("No idle time specified");
			return;
		}
	};

	let mut idle_time: u32 = 0;
	loop {
		let mut activity_flag = false;
		for check in checks {
			if check.run() {
				activity_flag = true;
				break;
			}
		}
		if activity_flag {
			idle_time = 0;
		} else {
			idle_time += interval;
		}
		if idle_time >= idle_limit {
			info!("Suspending");
			match suspend_cmd.status() {
				Ok(_) => {},
				Err(_) => {
					error!("Failed to suspend");
				}
			}
			idle_time = 0;
		}
		tokio::time::sleep(tokio::time::Duration::from_secs(interval.into())).await;
	}

}