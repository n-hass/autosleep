use crate::checks::{CheckType};
use std::collections::HashMap;
use std::process::Command;

use tokio::time::{interval, Duration};

use log::{info, error};

fn parse_constants(config: &HashMap<String, HashMap<String, Option<String>>>) -> (std::process::Command, u32, u32) {
	let suspend_cmd_field = match config.get("general").unwrap().get("suspend_cmd") {
		Some(suspend_cmd) => suspend_cmd,
		None => {
			error!("No suspend command field");
			panic!();
		}
	};
	let suspend_cmd: Command = match suspend_cmd_field {
		Some(cmd) => Command::new(cmd),
		None => {
			error!("No suspend command specified");
			panic!();
		}
	};

	let interval_field = match config.get("general").unwrap().get("interval") {
		Some(interval) => interval,
		None => {
			error!("No interval field");
			panic!();
		}
	};
	let interval: u32 = match interval_field {
		Some(interval) => {
			match interval.parse::<u32>() {
				Ok(interval) => interval,
				Err(_) => {
					error!("Invalid interval specified");
					panic!();
				}
			}
		},
		None => {
			error!("No interval specified");
			panic!();
		}
	};

	let idle_field = match config.get("general").unwrap().get("idle_time") {
		Some(idle_time) => idle_time,
		None => {
			error!("No idle time field");
			panic!();
		}
	};
	let idle_limit: u32 = match idle_field {
		Some(idle_time) => {
			match idle_time.parse::<u32>() {
				Ok(idle_time) => idle_time,
				Err(_) => {
					error!("Invalid idle time specified");
					panic!();
				}
			}
		},
		None => {
			error!("No idle time specified");
			panic!();
		}
	};

	return (
		suspend_cmd,
		interval,
		idle_limit
	);
}

pub async fn d_loop(checks: &Vec<Box<dyn CheckType>>, config: &HashMap<String, HashMap<String, Option<String>>>) {
	let (mut suspend_cmd, interval_time, idle_limit) = parse_constants(config);

	let mut idle_time: u32 = 0;
	let mut interval = interval(Duration::from_secs(interval_time.into()));
	loop {
		interval.tick().await;
    let mut successful_check = None;

    for check in checks {
			if check.run() {
				successful_check = Some(check.get_check_name());
				break;
			}
    }

    if let Some(name) = successful_check {
			info!("Check \"{}\" reports activity", name);
			idle_time = 0;
    } else {
			idle_time += interval_time;
    }

    if idle_time >= idle_limit {
			info!("Idle time limit reached. Suspending ...");
			idle_time = 0;
			if suspend_cmd.status().is_err() {
				error!("Failed to suspend");
			}
    }
	}

}