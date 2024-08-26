use std::env;
use std::path::Path;
use std::path::PathBuf;

use proc_macro2::Span;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Taken from <https://github.com/launchbadge/sqlx/blob/f69f370f25f099fd5732a5383ceffc76f724c482/sqlx-macros-core/src/common.rs#L1C1-L37C2>
pub fn resolve_path(path: impl AsRef<Path>, error_span: Span) -> syn::Result<PathBuf> {
	let path = path.as_ref();

	if path.is_absolute() {
		return Err(syn::Error::new(
			error_span,
			"absolute paths will only work on the current machine",
		));
	}

	// requires `proc_macro::SourceFile::path()` to be stable
	// https://github.com/rust-lang/rust/issues/54725
	if path.is_relative()
		&& !path
			.parent()
			.map_or(false, |parent| !parent.as_os_str().is_empty())
	{
		return Err(syn::Error::new(
			error_span,
			"paths relative to the current file's directory are not currently supported",
		));
	}

	let base_dir = env::var("CARGO_MANIFEST_DIR").map_err(|_| {
		syn::Error::new(
			error_span,
			"CARGO_MANIFEST_DIR is not set; please use Cargo to build",
		)
	})?;
	let base_dir_path = Path::new(&base_dir);

	Ok(base_dir_path.join(path))
}

pub async fn rustfmt(source: &str) -> std::io::Result<String> {
	let mut process = Command::new("rustfmt")
		.args([
			"+nightly",
			"--emit",
			"stdout",
			"--unstable-features",
			"--edition",
			"2021",
		])
		.env("RUSTUP_TOOLCHAIN", "nightly")
		.stdin(std::process::Stdio::piped())
		.stdout(std::process::Stdio::piped())
		.spawn()?;

	let mut stdin = process.stdin.take().unwrap();
	stdin.write_all(source.as_bytes()).await?;
	stdin.flush().await?;
	drop(stdin);

	String::from_utf8(process.wait_with_output().await?.stdout).map_err(|_| {
		std::io::Error::new(std::io::ErrorKind::InvalidData, "Rustfmt output not UTF-8")
	})
}
