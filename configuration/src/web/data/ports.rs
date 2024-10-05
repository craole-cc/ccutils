use anyhow::{anyhow, Result};
use std::fmt::{self, Display, Formatter};
use tword_util::envs::getenv_or;

#[derive(Debug, Clone, Copy)]
pub struct Ports {
	pub http: u16,
	pub https: u16,
}

impl Display for Ports {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "HTTP: {}", self.http)?;
		writeln!(f, "HTTPS: {}", self.https)?;
		Ok(())
	}
}

impl Default for Ports {
	fn default() -> Self {
		let port = getenv_or("SERVICE_WEB_PORT", 8080).expect("msg");
		let port_a =
			getenv_or("SERVICE_WEB_PORT_HTTPS", port).expect("msg");
		let port_b = getenv_or("SERVICE_WEB_PORT_HTTP", port_a + 1)
			.expect("msg");

		Self {
			http: port_b,
			https: port_a,
		}
	}
}
