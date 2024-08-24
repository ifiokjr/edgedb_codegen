use std::io::{self};
use std::process::Stdio;

use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::TokenStreamExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub async fn rustfmt(source: &str) -> io::Result<String> {
	let mut process = Command::new("rustfmt")
		.arg("--emit=stdout")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	let mut stdin = process.stdin.take().unwrap();
	stdin.write_all(source.as_bytes()).await?;
	stdin.flush().await?;
	drop(stdin);

	String::from_utf8(process.wait_with_output().await?.stdout)
		.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Rustfmt output not UTF-8"))
}

pub struct Char(char);
impl ToTokens for Char {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Punct::new(self.0, Spacing::Alone));
	}
}

pub const POUND: Char = Char('#');
