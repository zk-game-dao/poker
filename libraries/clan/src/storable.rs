use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use candid::{Decode, Encode};
use currency::Currency;
use ic_stable_structures::{Storable, storable::Bound};
use user::user::WalletPrincipalId;

use crate::{
    Clan, ClanId, ClanPrivacy, ClanStats, environment::ClanEnvironmentSettings,
    treasury::ClanTreasury,
};

const MAX_CLAN_SIZE: u32 = 50_000_000; // 50MB max size for clan data

// Implement Storable for Clan to work with ic_stable_structures
impl Storable for Clan {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Clan serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Clan deserialization error: {:?}", e);
            // Return a default clan that should not be used
            Clan {
                id: ClanId::default(),
                name: "ERROR".to_string(),
                description: "".to_string(),
                tags: HashSet::new(),
                avatar: None,
                supported_currency: Currency::ICP,
                members: HashMap::new(),
                member_limit: 0,
                pending_requests: Vec::new(),
                invited_users: HashMap::new(),
                privacy: ClanPrivacy::Public,
                require_proof_of_humanity: false,
                subscription_enabled: false,
                subscription_tiers: HashMap::new(),
                joining_fee: 0,
                treasury: ClanTreasury::default(),
                environment_settings: ClanEnvironmentSettings::default(),
                stats: ClanStats::default(),
                active_tables: Vec::new(),
                hosted_tournaments: Vec::new(),
                created_at: 0,
                created_by: WalletPrincipalId::default(),
                website: None,
                discord: None,
                twitter: None,
            }
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_CLAN_SIZE,
        is_fixed_size: false,
    };
}
