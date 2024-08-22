use std::io::Write;
use std::io::{self};
use std::process::Command;
use std::process::Stdio;

pub fn rustfmt(source: &str) -> io::Result<String> {
	let mut process = Command::new("rustfmt")
		.arg("--emit=stdout")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	let mut stdin = process.stdin.take().unwrap();
	stdin.write_all(source.as_bytes())?;
	stdin.flush()?;
	drop(stdin);

	String::from_utf8(process.wait_with_output()?.stdout)
		.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Rustfmt output not UTF-8"))
}
