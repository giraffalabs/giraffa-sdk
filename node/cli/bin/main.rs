//! Giraffa SDK CLI library.

#![warn(missing_docs)]

pub use sc_cli::{VersionInfo, IntoExit, error};

fn main() -> Result<(), cli::error::Error> {
	let version = VersionInfo {
		name: "Giraffa SDK",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "giraffa-sdk",
		author: "Giraffa Labs",
		description: "Giraffa SDK template node",
		support_url: "giraffa.io",
	};

	cli::run(std::env::args(), cli::Exit, version)
}
