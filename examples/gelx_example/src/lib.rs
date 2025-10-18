mod db;

pub use db::*;

#[cfg(test)]
mod tests {
	use gelx::exports::bigdecimal::BigDecimal;
	use gelx::exports::uuid::Uuid;

	use super::*;

	#[tokio::test]
	async fn test_select_user_query() -> anyhow::Result<()> {
		let client = Globals::builder()
			.current_user_id(Uuid::max())
			.alternative("test")
			.build()
			.into_client()
			.await?;
		let props = select_user::Input::builder().slug("test").build();
		select_user::query(&client, &props).await?;
		Ok(())
	}

	#[tokio::test]
	async fn test_select_accounts_query() -> anyhow::Result<()> {
		let client = Globals::builder()
			.current_user_id(Uuid::max())
			.alternative("test")
			.build()
			.into_client()
			.await?;
		let props = select_accounts::Input::builder()
			.provider(AccountProvider::Github)
			.build();
		select_accounts::query(&client, &props).await?;
		Ok(())
	}

	#[tokio::test]
	async fn test_insert_position_query() -> anyhow::Result<()> {
		let client = Globals::builder()
			.current_user_id(Uuid::max())
			.alternative("test")
			.build()
			.into_client()
			.await?;
		let props = insert_position::Input::builder().position(1).build();
		insert_position::query(&client, &props).await?;
		Ok(())
	}

	#[tokio::test]
	async fn test_select_test_user_query() -> anyhow::Result<()> {
		let client = Globals::builder()
			.current_user_id(Uuid::max())
			.alternative("test")
			.build()
			.into_client()
			.await?;
		let props = select_test_user::Input::builder()
			.username("custom")
			.build();
		select_test_user::query(&client, &props).await?;
		Ok(())
	}

	#[tokio::test]
	async fn test_select_transactions_query() -> anyhow::Result<()> {
		let client = Globals::builder()
			.current_user_id(Uuid::max())
			.alternative("test")
			.build()
			.into_client()
			.await?;
		let props = select_transactions::Input::builder()
			.amount(BigDecimal::from(0))
			.build();
		select_transactions::query(&client, &props).await?;
		Ok(())
	}
}
