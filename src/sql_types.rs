#[derive(sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum SQLRole {
    Regular,
    Admin
}

#[derive(sqlx::FromRow)]
pub struct SQLUser {
    pub name: String,
    pub pass_hash: String,
    pub role: SQLRole
}

use crate::paseto::PASETORole as PasetoRole;

impl From<SQLRole> for PasetoRole {
    fn from(value: SQLRole) -> Self {
        match value {
            SQLRole::Admin => PasetoRole::Admin,
            SQLRole::Regular => PasetoRole::Regular
        }
    }
}

/*
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(sqlx::FromRow)]
struct Node {
    id: Uuid,
    continent: String,
    country: String,
    region: String,
    city: String,
    district: String,
    lat: f64,
    lon: f64,
    alt_m: f32,
    last_heartbeat: OffsetDateTime
}

#[derive(sqlx::FromRow)]
pub struct Log {
    temp:f32,
    hum:f32,
    air_q:f32,
    timestamp:OffsetDateTime,
    node_id: Uuid,
}
*/