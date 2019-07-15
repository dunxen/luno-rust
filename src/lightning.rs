use serde::Deserialize;
use std::collections::HashMap;

use crate::{client, Currency};

#[derive(Debug, Deserialize)]
pub struct LightningWithdrawal {
    pub invoice_id: String,
    pub payment_request: String,
}

#[derive(Debug, Deserialize)]
pub struct LightningReceiveRequest {
    pub invoice_id: String,
    pub payment_request: String,
}

pub struct LightningSendBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) params: HashMap<&'a str, String>,
}

impl<'a> LightningSendBuilder<'a> {
    pub fn with_currency(&mut self, currency: Currency) -> &mut LightningSendBuilder<'a> {
        self.params.insert("currency", currency.to_string());
        self
    }

    pub fn with_description(&mut self, description: &'a str) -> &mut LightningSendBuilder<'a> {
        self.params.insert("description", description.to_string());
        self
    }

    pub fn with_external_id(&mut self, external_id: &'a str) -> &mut LightningSendBuilder<'a> {
        self.params.insert("external_id", external_id.to_string());
        self
    }

    pub fn send(&self) -> Result<LightningWithdrawal, reqwest::Error> {
        let url = self.url.clone();
        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()?
            .json()
    }
}

pub struct LightningReceiveBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) params: HashMap<&'a str, String>,
}

impl<'a> LightningReceiveBuilder<'a> {
    pub fn with_currency(&mut self, currency: Currency) -> &mut LightningReceiveBuilder<'a> {
        self.params.insert("currency", currency.to_string());
        self
    }

    pub fn with_description(&mut self, description: &'a str) -> &mut LightningReceiveBuilder<'a> {
        self.params.insert("description", description.to_string());
        self
    }

    pub fn create(&self) -> Result<LightningReceiveRequest, reqwest::Error> {
        let url = self.url.clone();
        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()?
            .json()
    }
}
