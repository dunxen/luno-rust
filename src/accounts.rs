use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::market::Currency;
use crate::{client, error::LunoError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
	pub id: String,
	pub currency: Currency,
	pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
	pub account_id: String,
	pub asset: Currency,
	pub balance: Decimal,
	pub reserved: Decimal,
	pub unconfirmed: Decimal,
	pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListBalancesResponse {
	pub balance: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountNameResponse {
	pub success: bool,
}
pub struct ListBalancesBuilder<'a> {
	pub(crate) luno_client: &'a client::LunoClient,
	pub(crate) url: reqwest::Url,
	pub(crate) assets: Option<&'a [Currency]>,
}

impl<'a> ListBalancesBuilder<'a> {
	pub fn with_assets(&mut self, assets: &'a [Currency]) -> &mut ListBalancesBuilder<'a> {
		self.assets = Some(assets);
		self
	}

	pub async fn list(&self) -> Result<Vec<Balance>, LunoError> {
		let mut url = self.url.clone();
		if let Some(assets) = self.assets {
			url.query_pairs_mut().append_pair(
				"assets",
				&assets
					.iter()
					.map(|c| c.to_string())
					.collect::<Vec<String>>()
					.join(","),
			);
		}
		Ok(self
			.luno_client
			.get::<ListBalancesResponse>(url)
			.await?
			.balance)
	}
}
