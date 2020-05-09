use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Beneficiary {
    bank_account_branch: String,
    bank_account_number: String,
    bank_account_type: String,
    bank_country: String,
    bank_name: String,
    bank_recipient: String,
    created_at: u64,
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListBeneficiariesResponse {
    beneficiaries: Vec<Beneficiary>,
}
