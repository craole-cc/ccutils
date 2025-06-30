use crate::prelude::*;

pub fn init() -> Result<()> {
  let mut logger = logline::Config::default();
  // logger = logger
  //   // .with_level(logline::Level::TRACE)
  //   // .hide_level()
  //   // .use_env()
  //   // .show_target()
  //   // .show_line()
  //   // .with_time(logline::Time::Uptime)
  //   // .plain();
  //   .pretty();
  logger.init();
  trace!("Logline \n{:#?}", &logger);
  info!("Initialized logging for {APP} v.{VERSION}");
  Ok(())
}
