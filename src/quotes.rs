use std::collections::HashMap;

use reqwest::Url;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{error::LunoError, LunoClient, MarketOrderType, TradingPair};

#[derive(Debug, Deserialize)]
pub struct Quote {
	pub base_amount: Decimal,
	pub counter_amount: Decimal,
	pub created_at: u64,
	pub discarded: bool,
	pub exercised: bool,
	pub expires_at: bool,
	pub id: String,
	pub pair: TradingPair,
	#[serde(alias = "type")]
	pub order_type: MarketOrderType,
}

pub struct CreateQuoteBuilder<'a> {
	pub(crate) luno_client: &'a LunoClient,
	pub(crate) url: Url,
	pub(crate) params: HashMap<&'a str, String>,
}

impl<'a> CreateQuoteBuilder<'a> {
	pub fn with_base_account(&mut self, id: &'a str) -> &mut CreateQuoteBuilder<'a> {
		self.params.insert("base_account_id", id.to_string());
		self
	}

	pub fn with_counter_account(&mut self, id: &'a str) -> &mut CreateQuoteBuilder<'a> {
		self.params.insert("counter_account_id", id.to_string());
		self
	}

	pub async fn post(&mut self) -> Result<Quote, LunoError> {
		let url = self.url.clone();

		Ok(self
			.luno_client
			.http
			.post(url)
			.basic_auth(
				self.luno_client.credentials.key.to_owned(),
				Some(self.luno_client.credentials.secret.to_owned()),
			)
			.form(&self.params)
			.send()
			.await?
			.json()
			.await?)
	}
}
