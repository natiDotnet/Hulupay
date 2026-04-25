use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Invited,
    Suspended,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct KpiCard {
    pub label: &'static str,
    pub value: &'static str,
    pub delta: &'static str,
    pub positive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartPoint {
    pub label: &'static str,
    pub value: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    pub id: String,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SystemConfig {
    pub allow_signups: bool,
    pub default_role: Role,
    pub notify_security_events: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProfileSettings {
    pub full_name: String,
    pub timezone: String,
    pub locale: String,
}
