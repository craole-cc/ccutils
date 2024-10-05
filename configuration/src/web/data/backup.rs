use crate::{ConfigTrait, ProjectConfig};
use anyhow::{anyhow, Result};
use std::{
	fmt::{self, Display, Formatter},
	net::{
		IpAddr::{self, V4},
		Ipv4Addr, SocketAddr,
	},
	path::PathBuf,
};
use tword_auth::tls::{generate, CertificateInfo, CertificateKeys};
use tword_util::{
	envs::{getenv_or, getenv_path},
	string::capitalize_words,
};

impl ConfigTrait for Config {
	const NAME: &'static str = "Web";
	async fn load_data() -> Result<Self> {
		let host = Self::get_host()?;
		let socket_address = Self::get_socket_addr()?;
		let assets_path = Self::get_assets_path()?;
		let tls_certificate = Self::get_tls_certificate()?;
		let tls_private_key = Self::get_tls_private_key()?;

		Ok(Self {
			host,
			ports: Ports::default(),
			assets_path,
			socket_address,
			tls_certificate,
			tls_private_key,
		})
	}
}

#[derive(Debug)]
pub struct Config {
	pub host: String,
	pub ports: Ports,
	pub socket_address: SocketAddr,
	pub assets_path: PathBuf,
	pub tls_certificate: PathBuf,
	pub tls_private_key: PathBuf,
}

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

impl Ports {
	fn default() -> Self {
		let port = getenv_or("SERVICE_WEB_PORT", 8080);
		let port_a = getenv_or("SERVICE_WEB_PORT_HTTPS", port);
		let port_b = getenv_or("SERVICE_WEB_PORT_HTTP", port_a + 1);

		Self {
			http: port_b,
			https: port_a,
		}
	}
}

impl Display for Config {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tInfo:")?;
		writeln!(f, "\t  Host: {}", self.host)?;
		writeln!(f, "\t  Ports: {}", self.ports)?;
		writeln!(f, "\t  Address: {}", self.socket_address)?;
		writeln!(f, "\tPaths:")?;
		writeln!(
			f,
			"\t  Assets Path: {}",
			self.assets_path.display()
		)?;
		writeln!(
			f,
			"\t  TLS Certificate Path: {}",
			self.tls_certificate.display()
		)?;
		writeln!(
			f,
			"\t  TLS Private Key Path: {}",
			self.tls_private_key.display()
		)?;
		Ok(())
	}
}

impl Config {
	fn get_host() -> Result<String> {
		let host =
			getenv_or("SERVICE_WEB_HOST", "localhost".to_string());
		Ok(host)
	}

	fn get_socket_addr() -> Result<SocketAddr> {
		let host_var = Self::get_host()?;
		let host = match host_var.as_str() {
			"localhost" => V4(Ipv4Addr::LOCALHOST),
			"unspecified" => V4(Ipv4Addr::UNSPECIFIED),
			_ => host_var.parse::<IpAddr>().map_err(|_| {
				anyhow!(
					"Failed to parse SERVICE_WEB_HOST ({:#?}) as a valid host.",
					host_var
				)
			})?,
		};
		let port = Self::get_port()?;
		let socket_address = SocketAddr::new(host, port);
		Ok(socket_address)
	}

	fn get_assets_path() -> Result<PathBuf> {
		Ok(getenv_path("SERVICE_WEB_ASSETS", "assets/web")?)
	}

	fn get_tls_name() -> Result<String> {
		let project_root_path = &Project::get_root_path()?;
		let project_name = project_root_path
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.into_owned();
		let project_pretty_name =
			capitalize_words(project_name.replace("_", " "));

		Ok(getenv_or("SERVICE_WEB_TLS_NAME", project_pretty_name))
	}

	fn get_tls_org() -> Result<String> {
		Ok(getenv_or("SERVICE_WEB_TLS_ORG", "".to_string()))
	}

	fn get_tls_unit() -> Result<String> {
		Ok(getenv_or("SERVICE_WEB_TLS_UNIT", "".to_string()))
	}

	fn get_tls_city() -> Result<String> {
		Ok(getenv_or("SERVICE_WEB_TLS_CITY", "".to_string()))
	}

	fn get_tls_state() -> Result<String> {
		Ok(getenv_or("SERVICE_WEB_TLS_STATE", "".to_string()))
	}

	fn get_tls_country() -> Result<String> {
		Ok(getenv_or("SERVICE_WEB_TLS_COUNTRY", "".to_string()))
	}

	fn regenerate_tls() -> bool {
		getenv_or("SERVICE_WEB_REGENERATE_TLS", false)
	}

	fn generate_tls() -> Result<CertificateKeys> {
		let keys = generate(
			Self::NAME,
			&Self::get_assets_path()?.join("keys"),
			&CertificateInfo {
				name: &Self::get_tls_name()?,
				org: &Self::get_tls_org()?,
				unit: &Self::get_tls_unit()?,
				city: &Self::get_tls_city()?,
				state: &Self::get_tls_state()?,
				country: &Self::get_tls_country()?,
			},
			Self::regenerate_tls(),
		)?;

		Ok(keys)
	}

	fn get_tls_certificate() -> Result<PathBuf> {
		Ok(Self::generate_tls()?.certificate_pem)
	}

	fn get_tls_private_key() -> Result<PathBuf> {
		Ok(Self::generate_tls()?.private_key_pem)
	}
}
