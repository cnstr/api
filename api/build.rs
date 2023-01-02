use anyhow::Result;
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<()> {
	// VERGEN_BUILD_TIMESTAMP
	// VERGEN_BUILD_SEMVER

	// VERGEN_GIT_BRANCH
	// VERGEN_GIT_SHA_SHORT

	// VERGEN_RUSTC_HOST_TRIPLE
	// VERGEN_RUSTC_LLVM_VERSION
	// VERGEN_RUSTC_SEMVER

	let mut config = Config::default();
	*config.git_mut().sha_kind_mut() = ShaKind::Short;

	vergen(config)?;
	return Ok(());
}
