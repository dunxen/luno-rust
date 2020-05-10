use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Beneficiary {
    pub bank_account_branch: String,
    pub bank_account_number: String,
    pub bank_account_type: String,
    pub bank_country: String,
    pub bank_name: String,
    pub bank_recipient: String,
    pub created_at: u64,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListBeneficiariesResponse {
    beneficiaries: Vec<Beneficiary>,
}
