use std::{
  fmt::Debug,
  path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "download-server",
  about = "Download Server"
)]
pub struct Arguments {
  /// Activate debug mode
  #[structopt(long)]
  pub debug: bool,

  /// Service port number
  #[structopt(long, default_value = "50051")]
  pub service_port: u16,

  /// Activate stdout logging
  #[structopt(long)]
  pub stdout_log: bool,

  /// log file path
  #[structopt(long, parse(from_os_str))]
  pub log_file: Option<PathBuf>,
}

impl Arguments {
  pub fn get() -> Self {
    Arguments::from_args()
  }
}
