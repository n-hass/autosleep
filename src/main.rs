use std::path::PathBuf;
use clap::{Parser};
use configparser::ini::Ini;
use std::error::Error;

use log::{info, warn, debug, error, LevelFilter};
use systemd_journal_logger::JournalLog;

mod daemon;

mod checks;
use crate::checks::*;

mod setup;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short = 'c', long)]
  config: Option<PathBuf>,

  #[arg(long)]
  debug: bool,
  #[arg(short = 'C', long)]
  check_all: bool,
}

#[tokio::main]
async fn main() {
  JournalLog::default().install().unwrap();
  
  let args = Args::parse();
  if args.debug { log::set_max_level(LevelFilter::Debug); }
  else          { log::set_max_level(LevelFilter::Info);  }

  let config_path = match args.config {
    Some(path) => path,
    None => PathBuf::from("/etc/autosleep.d/autosleep.conf"), // default
  };

  if config_path.exists() {
    debug!("Config file found at {:?}", config_path);
  } else {
    // create a default config here maybe?
    error!("No config file at {:?}", config_path);
  }
  
  let mut configFile = Ini::new();

  let config = match configFile.load(config_path) {
    Ok(config) => config,
    Err(e) => {
      error!("Error loading config file: {}", e);
      return;
    }
  };

  let mut checks: Vec<Box<dyn CheckFn>> = Vec::new();

  setup::create_checks(&mut checks, &config);

  daemon::d_loop(&checks, &config).await;

}
