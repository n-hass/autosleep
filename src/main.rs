use std::path::PathBuf;
use clap::{Parser};
use configparser::ini::Ini;

use log::{LevelFilter};

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
  let args = Args::parse();

  let config_path = match args.config {
    Some(path) => path,
    None => PathBuf::from("/etc/autosleep.d/autosleep.conf"), // default
  };
  if config_path.exists() == false {
    panic!("No config file at {:?}", config_path);
  }
  let mut config_file = Ini::new();
  let config = match config_file.load(config_path) {
    Ok(config) => config,
    Err(e) => {
      panic!("Error loading config file: {}", e);
    }
  };
  let general_config = match config.get("general") {
    Some(general_config) => general_config,
    None => {
      panic!("No general config section");
    }
  };

  if args.debug {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }
  setup::install_logger(general_config);
  log::info!("log test");
  
  let mut checks: Vec<Box<dyn CheckType>> = Vec::new();

  setup::create_checks(&mut checks, &config);

  daemon::d_loop(&checks, &config).await;

}
