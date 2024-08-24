use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub async fn rustfmt(source: &str) -> std::io::Result<String> {
	let mut process = Command::new("rustfmt")
		.arg("--emit=stdout")
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
