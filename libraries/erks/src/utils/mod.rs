use crate::{
	debug, error, info, trace, warn, Core, DEBUG, ERROR, INFO, TRACE,
	WARN,
};

pub fn init() {
	Core::default().init();
}

pub fn init_trace() {
	Core::default().with_level(TRACE).init();
}

pub fn init_debug() {
	Core::default().with_level(DEBUG).init();
}

pub fn init_info() {
	Core::default().with_level(INFO).init();
}

pub fn init_warnings() {
	Core::default().with_level(WARN).init();
}

pub fn init_errors() {
	Core::default().with_level(ERROR).init();
}

pub fn test() {
	init_trace();
	trace!("Tracing initialized via logline!");
	debug!("Tracing initialized via logline!");
	info!("Tracing initialized via logline!");
	warn!("Tracing initialized via logline!");
	error!("Tracing initialized via logline!");
}
