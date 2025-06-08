use authentication::validate_caller;
use candid::Principal;
use canister_functions::cycle::check_and_top_up_canister;
use errors::user_error::UserError;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use ic_verifiable_credentials::{
    issuer_api::CredentialSpec, validate_ii_presentation_and_claims, VcFlowSigners,
};
use lazy_static::lazy_static;
use user::user::{User, UserAvatar};

use std::sync::Mutex;

mod memory;

const MINIMUM_CYCLE_THRESHOLD: u128 = 350_000_000_000;

/// A structure to hold canister settings or state.
pub struct CanisterState {
    pub owner: Principal,
    pub default_subaccount: Subaccount,
    pub account_identifier: AccountIdentifier,
}

// Define a global instance of GameState wrapped in a Mutex for safe concurrent access.
lazy_static! {
    static ref CANISTER_STATE: Mutex<Option<CanisterState>> = Mutex::new(None);
    static ref ICP_LEDGER_CANISTER_ID: Principal =
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    static ref SUPPORT_US_WALLET: Principal =
        Principal::from_text("amwxf-a2rkd-b42qc-jwbst-oy3co-d5ues-jgfcp-khbg4-zdxoa-n66ja-2ae")
            .unwrap();

    static ref CONTROLLER_PRINCIPALS: Vec<Principal> = vec![
        Principal::from_text("km7qz-4bai4-e5ptx-hgrck-z3web-ameqg-ksxcf-u7wbr-t5fna-i7bqp-hqe").unwrap(),
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae").unwrap(),
    ];
    static ref USER_INDEX_PRINCIPAL: Mutex<Option<Principal>> = Mutex::new(None);

    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

fn handle_cycle_check() {
    let cycles = ic_cdk::api::canister_cycle_balance();
    if cycles >= MINIMUM_CYCLE_THRESHOLD {
        return;
    }
    ic_cdk::futures::spawn(async {
        let user_index_result = USER_INDEX_PRINCIPAL.lock();
        let user_index = match user_index_result {
            Ok(lock) => match *lock {
                Some(index) => index,
                None => {
                    ic_cdk::println!("User not found");
                    return; // or perform some error handling
                }
            },
            Err(_) => {
                ic_cdk::println!("Lock error occurred");
                return; // or handle the lock error
            }
        };

        if let Err(e) =
            check_and_top_up_canister(ic_cdk::api::canister_self(), user_index, MINIMUM_CYCLE_THRESHOLD).await
        {
            ic_cdk::println!("Failed to top up canister: {:?}", e);
        }
    });
}

#[ic_cdk::init]
fn init() {
    let principal = ic_cdk::api::canister_self();
    ic_cdk::println!(
        "Users canister {} initialized",
        principal.to_text()
    );
}

#[ic_cdk::query]
fn ping() -> String {
    "Ok".to_string()
}

#[ic_cdk::update]
fn create_user(
    user_name: String,
    address: Option<String>,
    internet_identity_principal_id: Principal,
    avatar: Option<UserAvatar>,
    eth_wallet_address: Option<String>,
    referrer: Option<Principal>,
) -> Result<(User, usize), UserError> {
    if user_name.is_empty() {
        return Err(UserError::InvalidRequest(
            "User name cannot be empty".to_string(),
        ));
    }

    if USER_INDEX_PRINCIPAL
        .lock()
        .map_err(|_| UserError::LockError)?
        .is_none()
    {
        *USER_INDEX_PRINCIPAL
            .lock()
            .map_err(|_| UserError::LockError)? = Some(ic_cdk::api::msg_caller());
    }

    handle_cycle_check();

    let user = User::new(
        internet_identity_principal_id,
        ic_cdk::api::canister_self(),
        user_name,
        0,
        address,
        avatar,
        eth_wallet_address,
        referrer,
        Some(ic_cdk::api::time()),
    );

    let mut user_state = USERS.lock().map_err(|_| UserError::LockError)?;
    user_state.push(user.clone());
    Ok((user, user_state.len()))
}

#[ic_cdk::update]
#[allow(clippy::too_many_arguments)]
fn update_user(
    user_id: Principal,
    user_name: Option<String>,
    balance: Option<u64>,
    address: Option<String>,
    avatar: Option<UserAvatar>,
    enlarge_text: Option<bool>,
    volume_level: Option<u16>,
    eth_wallet_address: Option<String>,
) -> Result<User, UserError> {
    handle_cycle_check();
    let mut user = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = user
        .iter_mut()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    let user_index = (*USER_INDEX_PRINCIPAL
        .lock()
        .map_err(|_| UserError::LockError)?)
    .unwrap_or(Principal::from_text("zpqcd-cyaaa-aaaam-qbe3q-cai").unwrap()); // The user principal in the canister_ids.json
    validate_caller(vec![user.principal_id, user_index]);
    if let Some(user_name) = user_name {
        user.set_user_name(user_name);
    }
    if let Some(balance) = balance {
        user.set_balance(balance);
    }
    if let Some(address) = address {
        user.set_address(Some(address));
    }
    if let Some(avatar) = avatar {
        user.set_avatar(Some(avatar));
    }
    if let Some(enlarge_text) = enlarge_text {
        user.enlarge_text = Some(enlarge_text);
    }
    if let Some(volume_level) = volume_level {
        user.volume_level = Some(volume_level);
    }
    if let Some(eth_wallet_address) = eth_wallet_address {
        user.eth_wallet_address = Some(eth_wallet_address);
    }
    Ok(user.clone())
}

#[ic_cdk::query]
fn get_user(user_id: Principal) -> Result<User, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user)
}

#[ic_cdk::update]
fn get_user_icc(user_id: Principal) -> Result<User, UserError> {
    handle_cycle_check();
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user)
}

#[ic_cdk::update]
fn add_active_table(table_principal: Principal, user_id: Principal) -> Result<User, UserError> {
    handle_cycle_check();

    let mut user = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = user
        .iter_mut()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    user.active_tables.push(table_principal);
    Ok(user.clone())
}

#[ic_cdk::update]
fn remove_active_table(table_principal: Principal, user_id: Principal) -> Result<User, UserError> {
    handle_cycle_check();

    let mut user = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = user
        .iter_mut()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    user.active_tables.retain(|table| *table != table_principal);
    Ok(user.clone())
}

#[ic_cdk::query]
fn get_active_tables(user_id: Principal) -> Result<Vec<Principal>, UserError> {
    handle_cycle_check();

    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user.active_tables)
}

#[ic_cdk::update]
fn add_experience_points(
    experience_points: u64,
    currency: String,
    user_id: Principal,
) -> Result<User, UserError> {
    handle_cycle_check();
    let mut user = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = user
        .iter_mut()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;

    if currency == *"BTC" {
        user.add_pure_poker_experience_points(experience_points);
    } else {
        user.add_experience_points(experience_points);
    }
    
    Ok(user.clone())
}

#[ic_cdk::update]
fn clear_experience_points() -> Result<(), UserError> {
    handle_cycle_check();
    let mut users = USERS.lock().map_err(|_| UserError::LockError)?;
    for user in users.iter_mut() {
        user.clear_experience_points();
    }

    Ok(())
}

#[ic_cdk::update]
fn clear_pure_poker_experience_points() -> Result<(), UserError> {
    handle_cycle_check();
    let mut users = USERS.lock().map_err(|_| UserError::LockError)?;
    for user in users.iter_mut() {
        user.clear_pure_poker_experience_points();
    }

    Ok(())
}

#[ic_cdk::query]
fn get_user_level(user_id: Principal) -> Result<f64, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user.get_level())
}

#[ic_cdk::query]
fn get_user_experience_points() -> Result<Vec<(Principal, u64)>, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let experience_points = user
        .into_iter()
        .map(|user| (user.principal_id, user.get_experience_points()))
        .collect();
    Ok(experience_points)
}

#[ic_cdk::query]
fn get_pure_poker_user_experience_points() -> Result<Vec<(Principal, u64)>, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let experience_points = user
        .into_iter()
        .map(|user| (user.principal_id, user.get_pure_poker_experience_points()))
        .collect();
    Ok(experience_points)
}

#[ic_cdk::query]
fn get_verified_user_experience_points() -> Result<Vec<(Principal, u64)>, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let experience_points = user
        .into_iter()
        .filter(|user| user.is_verified.unwrap_or(false))
        .map(|user| (user.principal_id, user.get_experience_points()))
        .collect();
    Ok(experience_points)
}

#[ic_cdk::query]
fn get_verified_pure_poker_user_experience_points() -> Result<Vec<(Principal, u64)>, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let experience_points = user
        .into_iter()
        .filter(|user| user.is_verified.unwrap_or(false))
        .map(|user| (user.principal_id, user.get_pure_poker_experience_points()))
        .collect();
    Ok(experience_points)
}

#[ic_cdk::query]
fn get_experience_points_by_uid(user_id: Principal) -> Result<u64, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user.get_experience_points())
}

#[ic_cdk::query]
fn get_pure_poker_experience_points_by_uid(user_id: Principal) -> Result<u64, UserError> {
    let user = USERS.lock().map_err(|_| UserError::LockError)?.clone();
    let user = user
        .into_iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    Ok(user.get_pure_poker_experience_points())
}

pub const IC_ROOT_KEY: &[u8; 133] = b"\x30\x81\x82\x30\x1d\x06\x0d\x2b\x06\x01\x04\x01\x82\xdc\x7c\x05\x03\x01\x02\x01\x06\x0c\x2b\x06\x01\x04\x01\x82\xdc\x7c\x05\x03\x02\x01\x03\x61\x00\x81\x4c\x0e\x6e\xc7\x1f\xab\x58\x3b\x08\xbd\x81\x37\x3c\x25\x5c\x3c\x37\x1b\x2e\x84\x86\x3c\x98\xa4\xf1\xe0\x8b\x74\x23\x5d\x14\xfb\x5d\x9c\x0c\xd5\x46\xd9\x68\x5f\x91\x3a\x0c\x0b\x2c\xc5\x34\x15\x83\xbf\x4b\x43\x92\xe4\x67\xdb\x96\xd6\x5b\x9b\xb4\xcb\x71\x71\x12\xf8\x47\x2e\x0d\x5a\x4d\x14\x50\x5f\xfd\x74\x84\xb0\x12\x91\x09\x1c\x5f\x87\xb9\x88\x83\x46\x3f\x98\x09\x1a\x0b\xaa\xae";

#[ic_cdk::update]
async fn verify_credential(
    user_id: Principal,
    vp_jwt: String,
    derivation_origin: String,
    effective_subject: Principal,
) -> Result<(), String> {
    let current_time_ns = ic_cdk::api::time(); // Get current IC time in nanoseconds
    let root_pk_raw = &IC_ROOT_KEY[IC_ROOT_KEY.len().saturating_sub(96)..];

    // Configure the verification parameters
    let vc_flow_signers = VcFlowSigners {
        // Internet Identity canister ID
        ii_canister_id: Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap(),
        ii_origin: "https://identity.ic0.app".to_string(),

        // Decide ID canister ID
        issuer_canister_id: Principal::from_text("qgxyr-pyaaa-aaaah-qdcwq-cai").unwrap(),
        issuer_origin: "https://id.decideai.xyz/".to_string(),
    };

    // The credential specification that matches what was requested
    let vc_spec = CredentialSpec {
        credential_type: "ProofOfUniqueness".to_string(),
        arguments: None,
    };

    // Verify the presentation and all included credentials
    validate_ii_presentation_and_claims(
        &vp_jwt,
        effective_subject,
        derivation_origin.to_string(),
        &vc_flow_signers,
        &vc_spec,
        root_pk_raw, // IC root public key for verifying canister signatures
        current_time_ns as u128,
    )
    .map_err(|e| format!("Verification failed: {:?}", e))?;

    // If verification succeeds, store the verified state
    // This is application-specific - implement based on your needs
    let mut user = USERS
        .lock()
        .map_err(|_| UserError::LockError)
        .map_err(|_| "Lock error")?;
    let user = user
        .iter_mut()
        .find(|user| user.principal_id == user_id)
        .ok_or("User not found")?;
    user.is_verified = Some(true);

    Ok(())
}

/// Referral system
#[ic_cdk::query]
fn get_referred_users(user_id: Principal) -> Result<Vec<Principal>, UserError> {
    let users = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = users
        .iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    
    Ok(user.referred_users.clone().unwrap_or_default())
}

#[ic_cdk::query]
fn get_referral_tier(user_id: Principal) -> Result<u8, UserError> {
    let users = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = users
        .iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    
    Ok(user.get_referral_tier())
}

#[ic_cdk::update]
fn get_referral_rake_percentage(user_id: Principal) -> Result<u8, UserError> {
    handle_cycle_check();
    let users = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = users
        .iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    
    Ok(user.get_referral_rake_percentage())
}

#[ic_cdk::update]
fn get_referrer(user_id: Principal) -> Result<Option<Principal>, UserError> {
    handle_cycle_check();
    let users = USERS.lock().map_err(|_| UserError::LockError)?;
    let user = users
        .iter()
        .find(|user| user.principal_id == user_id)
        .ok_or(UserError::UserNotFound)?;
    
    Ok(user.referrer)
}

ic_cdk::export_candid!();
