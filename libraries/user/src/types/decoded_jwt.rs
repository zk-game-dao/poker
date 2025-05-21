use candid::Principal;
use serde::Deserialize;
use serde_json::Value;
use errors::user_error::UserError;
use super::verifiable_credential::VerifiableCredential;

#[derive(Deserialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}

#[derive(Deserialize)]
struct JwtPayload {
    vc: Vec<Value>,  // Raw verifiable credentials
    iss: String,     // Issuer
    sub: String,     // Subject
    exp: u64,        // Expiration time
    iat: u64,        // Issued at time
}

pub struct DecodedJwt {
    claims: String,
    payload: JwtPayload,
}

impl DecodedJwt {
    pub fn get_verifiable_credentials(&self) -> Result<Vec<VerifiableCredential>, UserError> {
        let mut credentials = Vec::new();
        
        for vc in &self.payload.vc {
            let credential = VerifiableCredential {
                credential_type: vc["type"].as_array()
                    .ok_or(UserError::InvalidCredentialType("Invalid credential type".into()))?[1]
                    .as_str()
                    .ok_or(UserError::InvalidCredentialType("Invalid credential type string".into()))?
                    .to_string(),
                issuer: Principal::from_text(&self.payload.iss)
                    .map_err(|_| UserError::InvalidCredentialType("Invalid issuer principal".into()))?,
                issuance_date: vc["issuanceDate"].as_str()
                    .ok_or(UserError::InvalidCredentialType("Missing issuance date".into()))?
                    .to_string(),
                expiration_date: vc["expirationDate"].as_str()
                    .ok_or(UserError::InvalidCredentialType("Missing expiration date".into()))?
                    .to_string(),
                minimum_verification_date: vc["credentialSubject"]["minimumVerificationDate"].as_str()
                    .ok_or(UserError::InvalidCredentialType("Missing minimum verification date".into()))?
                    .to_string(),
                credential_subject: vc["credentialSubject"]["id"].as_str()
                    .ok_or(UserError::InvalidCredentialType("Missing credential subject".into()))?
                    .to_string(),
            };
            credentials.push(credential);
        }
        
        Ok(credentials)
    }

    pub async fn decode_and_verify_jwt(jwt: &str) -> Result<DecodedJwt, UserError> {
        // Split JWT into parts
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() != 3 {
            return Err(UserError::InvalidCredentialType("Invalid JWT format".into()));
        }

        // Decode header
        let header_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            parts[0]
        ).map_err(|_| UserError::InvalidCredentialType("Invalid header encoding".into()))?;
        
        let header: JwtHeader = serde_json::from_slice(&header_bytes)
            .map_err(|_| UserError::InvalidCredentialType("Invalid header format".into()))?;

        // Verify algorithm matches what Decide ID uses
        if header.alg != "EdDSA" {
            return Err(UserError::InvalidCredentialType("Invalid signature algorithm".into()));
        }

        // Decode payload
        let payload_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            parts[1]
        ).map_err(|_| UserError::InvalidCredentialType("Invalid payload encoding".into()))?;
        
        let payload: JwtPayload = serde_json::from_slice(&payload_bytes)
            .map_err(|_| UserError::InvalidCredentialType("Invalid payload format".into()))?;

        Ok(DecodedJwt {
            claims: parts[1].to_string(),
            payload,
        })
    }
}
