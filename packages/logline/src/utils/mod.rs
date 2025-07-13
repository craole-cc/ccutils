use crate::{Config, DEBUG, ERROR, INFO, Level, TRACE, WARN, debug, error, info, trace, warn};

pub fn init() {
  Config::default().init();
}

pub fn init_with_level(level: Level) {
  Config::default().with_level(level).init();
}

pub fn init_trace() {
  init_with_level(TRACE);
}

pub fn init_debug() {
  init_with_level(DEBUG);
}

pub fn init_info() {
  init_with_level(INFO);
}

pub fn init_warnings() {
  init_with_level(WARN);
}

pub fn init_errors() {
  init_with_level(ERROR);
}

pub fn test() {
  init_trace();
  trace!("Tracing initialized via logline!");
  debug!("Tracing initialized via logline!");
  info!("Tracing initialized via logline!");
  warn!("Tracing initialized via logline!");
  error!("Tracing initialized via logline!");
}
