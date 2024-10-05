use crate::{project, ConfigTrait};
use anyhow::{anyhow, Result};
use std::{
	env::var_os,
	fmt::{self, Display, Formatter},
	net::{
		IpAddr::{self, V4},
		Ipv4Addr, SocketAddr,
	},
	path::{Path, PathBuf},
};
use tword_auth::tls::{generate, CertificateInfo, CertificateKeys};
use tword_util::{
	envs::{getenv_or, getenv_path_or},
	string::capitalize_words,
};

impl ConfigTrait for Config {
	const NAME: &'static str = "Web";
	async fn load_data() -> Result<Self> {
		// | Project Config
		// let project_config = project::init().await?;
		// let pretty_name = project.pretty_name;

		// | Config
		// let host = Self::get_host()?;
		// let assets_path = Self::get_assets_path().await?;
		let assets_path = getenv_path_or(
			"SERVICE_WEB_ASSETS_PATH",
			PathBuf::from(
				var_os("CARGO_MANIFEST_DIR")
					.expect("Failed to get CARGO_MANIFEST_DIR"),
			)
			.join("assets")
			.join("web"),
		)?;
		let host = getenv_or("SERVICE_WEB_HOST", "localhost")?;

		// let socket_address = Self::get_socket_addr()?;
		// let tls_certificate = Self::get_tls_certificate()?;
		// let tls_private_key = Self::get_tls_private_key()?;

		Ok(Self {
			host,
			assets_path,
			// ports: Ports::default(),
			// socket_address,
			// tls_certificate,
			// tls_private_key,
		})
	}
}

#[derive(Debug)]
pub struct Config {
	pub assets_path: PathBuf,
	pub host: String,
	// pub ports: Ports,
	// pub socket_address: SocketAddr,
	// pub tls_certificate: PathBuf,
	// pub tls_private_key: PathBuf,
}

impl Display for Config {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tInfo:")?;
		writeln!(f, "\t  Host: {}", self.host)?;
		// writeln!(f, "\t  Ports: {}", self.ports)?;
		// writeln!(f, "\t  Address: {}", self.socket_address)?;
		writeln!(f, "\tPaths:")?;
		writeln!(f, "\t  Assets Path: {:?}", self.assets_path)?;
		// 		writeln!(
		// 			f,
		// 			"\t  TLS Certificate Path: {}",
		// 			self.tls_certificate.display()
		// 		)?;
		// 		writeln!(
		// 			f,
		// 			"\t  TLS Private Key Path: {}",
		// 			self.tls_private_key.display()
		// 		)?;
		Ok(())
	}
}

impl Config {
	fn get_assets_path() -> Result<PathBuf> {
		let env_path: &str = "SERVICE_WEB_ASSETS";
		let project_path = PathBuf::from(
			std::env::var_os("CARGO_MANIFEST_DIR").unwrap(),
		);
		let default_path = project_path.join("assets/web");
		let assets_path = getenv_path_or(env_path, default_path)?;

		Ok(assets_path)
	}

	// async fn get_assets_path() -> Result<PathBuf> {
	// 	let env_path = "SERVICE_WEB_ASSETS";
	// 	let project_config = project::init().await?;
	// 	let default_path = project_config.assets_path.join("web");
	// 	let assets_path = getenv_path_or(env_path, default_path)?;
	// 	Ok(assets_path)
	// }

	fn get_host() -> Result<String> {
		let host = getenv_or("SERVICE_WEB_HOST", "localhost")?;
		Ok(host)
	}

	// fn get_socket_addr() -> Result<SocketAddr> {
	// 	let host_var = Self::get_host()?;
	// 	let host = match host_var.as_str() {
	// 		"localhost" => V4(Ipv4Addr::LOCALHOST),
	// 		"unspecified" => V4(Ipv4Addr::UNSPECIFIED),
	// 		_ => host_var.parse::<IpAddr>().map_err(|_| {
	// 			anyhow!(
	// 				"Failed to parse SERVICE_WEB_HOST ({:#?}) as a valid host.",
	// 				host_var
	// 			)
	// 		})?,
	// 	};
	// 	let port = Self::get_port()?;
	// 	let socket_address = SocketAddr::new(host, port);
	// 	Ok(socket_address)
	// }

	// fn get_tls_name() -> Result<String> {
	// 	Ok(getenv_or(
	// 		"SERVICE_WEB_TLS_NAME",
	// 		"project_pretty_name".to_string(),
	// 	))
	// }

	// fn get_tls_org() -> Result<String> {
	// 	Ok(getenv_or("SERVICE_WEB_TLS_ORG", "".to_string()))
	// }

	// fn get_tls_unit() -> Result<String> {
	// 	Ok(getenv_or("SERVICE_WEB_TLS_UNIT", "".to_string()))
	// }

	// fn get_tls_city() -> Result<String> {
	// 	Ok(getenv_or("SERVICE_WEB_TLS_CITY", "".to_string()))
	// }

	// fn get_tls_state() -> Result<String> {
	// 	Ok(getenv_or("SERVICE_WEB_TLS_STATE", "".to_string()))
	// }

	// fn get_tls_country() -> Result<String> {
	// 	Ok(getenv_or("SERVICE_WEB_TLS_COUNTRY", "".to_string()))
	// }

	// fn regenerate_tls() -> bool {
	// 	getenv_or("SERVICE_WEB_REGENERATE_TLS", false)
	// }

	// fn generate_tls() -> Result<CertificateKeys> {
	// 	let keys = generate(
	// 		Self::NAME,
	// 		&Self::get_assets_path()?.join("keys"),
	// 		&CertificateInfo {
	// 			name: &Self::get_tls_name()?,
	// 			org: &Self::get_tls_org()?,
	// 			unit: &Self::get_tls_unit()?,
	// 			city: &Self::get_tls_city()?,
	// 			state: &Self::get_tls_state()?,
	// 			country: &Self::get_tls_country()?,
	// 		},
	// 		Self::regenerate_tls(),
	// 	)?;

	// 	Ok(keys)
	// }

	// fn get_tls_certificate() -> Result<PathBuf> {
	// 	Ok(Self::generate_tls()?.certificate_pem)
	// }

	// fn get_tls_private_key() -> Result<PathBuf> {
	// 	Ok(Self::generate_tls()?.private_key_pem)
	// }
}
