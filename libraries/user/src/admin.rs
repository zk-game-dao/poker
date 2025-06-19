use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::user::{time, User, WalletPrincipalId};

// Admin role hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub enum AdminRole {
    Moderator,
    Admin,
    SuperAdmin,
}

impl AdminRole {
    pub fn can_ban_users(&self) -> bool {
        matches!(self, AdminRole::Moderator | AdminRole::Admin | AdminRole::SuperAdmin)
    }

    pub fn can_promote_to_moderator(&self) -> bool {
        matches!(self, AdminRole::Admin | AdminRole::SuperAdmin)
    }

    pub fn can_promote_to_admin(&self) -> bool {
        matches!(self, AdminRole::SuperAdmin)
    }

    pub fn can_manage_admin(&self, target_role: &AdminRole) -> bool {
        match self {
            AdminRole::SuperAdmin => true,
            AdminRole::Admin => matches!(target_role, AdminRole::Moderator),
            AdminRole::Moderator => false,
        }
    }
}

// Different types of bans
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum BanType {
    /// User can play but cannot gain XP for a specified duration
    XpBan {
        reason: String,
        banned_by: WalletPrincipalId,
        banned_at: u64,
        expires_at: u64,
    },
    /// Temporary suspension - user cannot play for a specified duration
    TemporarySuspension {
        reason: String,
        banned_by: WalletPrincipalId,
        banned_at: u64,
        expires_at: u64,
    },
    /// Permanent ban - user cannot access the platform
    PermanentBan {
        reason: String,
        banned_by: WalletPrincipalId,
        banned_at: u64,
    },
}

impl BanType {
    pub fn is_active(&self) -> bool {
        let now = time();
        match self {
            BanType::XpBan { expires_at, .. } => now < *expires_at,
            BanType::PermanentBan { .. } => true,
            BanType::TemporarySuspension { expires_at, .. } => now < *expires_at,
        }
    }

    pub fn prevents_gameplay(&self) -> bool {
        match self {
            BanType::XpBan { .. } => false,
            BanType::TemporarySuspension { .. } => self.is_active(),
            BanType::PermanentBan { .. } => true,
        }
    }

    pub fn prevents_xp_gain(&self) -> bool {
        match self {
            BanType::XpBan { .. } => self.is_active(),
            BanType::TemporarySuspension { .. } => self.is_active(),
            BanType::PermanentBan { .. } => true,
        }
    }

    pub fn get_reason(&self) -> &str {
        match self {
            BanType::XpBan { reason, .. } => reason,
            BanType::TemporarySuspension { reason, .. } => reason,
            BanType::PermanentBan { reason, .. } => reason,
        }
    }

    pub fn get_banned_by(&self) -> &WalletPrincipalId {
        match self {
            BanType::XpBan { banned_by, .. } => banned_by,
            BanType::TemporarySuspension { banned_by, .. } => banned_by,
            BanType::PermanentBan { banned_by, .. } => banned_by,
        }
    }

    pub fn get_banned_at(&self) -> u64 {
        match self {
            BanType::XpBan { banned_at, .. } => *banned_at,
            BanType::TemporarySuspension { banned_at, .. } => *banned_at,
            BanType::PermanentBan { banned_at, .. } => *banned_at,
        }
    }
}

impl User {
    // Admin role management
    pub fn promote_to_role(&mut self, new_role: AdminRole) {
        self.admin_role = Some(new_role);
    }

    pub fn is_admin(&self) -> bool {
        !matches!(self.admin_role, None)
    }

    pub fn can_perform_admin_action(&self, target_role: &AdminRole) -> bool {
        if let Some(role) = &self.admin_role {
            return role.can_manage_admin(target_role);
        }
        false
    }

    // Ban system methods
    pub fn apply_ban(&mut self, ban: BanType) {
        // Archive current ban if exists
        if let Some(current_ban) = &self.ban_status {
            let ban_history = self.ban_history.get_or_insert_with(Vec::new);
            ban_history.push(current_ban.clone());
        }
        self.ban_status = Some(ban);
    }

    pub fn remove_ban(&mut self) {
        if let Some(ban) = self.ban_status.take() {
            let ban_history = self.ban_history.get_or_insert_with(Vec::new);
            ban_history.push(ban);
        }
    }

    pub fn is_banned(&self) -> bool {
        self.ban_status
            .as_ref()
            .map(|ban| ban.is_active())
            .unwrap_or(false)
    }

    pub fn can_play(&self) -> bool {
        !self.ban_status
            .as_ref()
            .map(|ban| ban.prevents_gameplay())
            .unwrap_or(false)
    }

    pub fn can_gain_xp(&self) -> bool {
        !self.ban_status
            .as_ref()
            .map(|ban| ban.prevents_xp_gain())
            .unwrap_or(false)
    }

    pub fn get_ban_info(&self) -> Option<&BanType> {
        self.ban_status.as_ref().filter(|ban| ban.is_active())
    }

    pub fn clean_expired_bans(&mut self) {
        if let Some(ban) = &self.ban_status {
            if !ban.is_active() {
                let expired_ban = self.ban_status.take().unwrap();
                let ban_history = self.ban_history.get_or_insert_with(Vec::new);
                ban_history.push(expired_ban);
            }
        }
    }
}
