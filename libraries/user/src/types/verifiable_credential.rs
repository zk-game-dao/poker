use candid::Principal;
use errors::user_error::UserError;

pub struct VerifiableCredential {
    pub credential_type: String,
    pub issuer: Principal,
    pub issuance_date: String,
    pub expiration_date: String,
    pub minimum_verification_date: String,
    pub credential_subject: String,
}

impl VerifiableCredential {
    pub fn get_minimum_verification_date(&self) -> Result<String, UserError> {
        Ok(self.minimum_verification_date.clone())
    }
}
