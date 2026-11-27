use time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum SQLRole {
    Regular,
    Admin,
}

#[derive(sqlx::FromRow)]
pub struct SQLUser {
    pub name: String,
    pub pass_hash: String,
    pub role: SQLRole,
}

#[derive(sqlx::FromRow)]
pub struct SQLog {
    pub timestamp: OffsetDateTime,
    pub temp: f32,
    pub hum: f32,
    pub air_q: f32,
}

#[derive(sqlx::FromRow)]
pub struct SQLNode {
    pub id: Uuid,
    pub continent: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub district: String,
}

use crate::paseto::PASETORole;

impl From<SQLRole> for PASETORole {
    fn from(value: SQLRole) -> Self {
        match value {
            SQLRole::Admin => PASETORole::Admin,
            SQLRole::Regular => PASETORole::Regular,
        }
    }
}

use crate::http_types::{Location, Log};

impl From<SQLog> for Log {
    fn from(value: SQLog) -> Self {
        Self {
            temp: value.temp,
            hum: value.hum,
            air_q: value.air_q,
            timestamp: value.timestamp,
        }
    }
}

impl From<SQLNode> for Location {
    fn from(value: SQLNode) -> Self {
        Self {
            continent: value.continent,
            country: value.country,
            region: value.region,
            city: value.city,
            district: value.district,
        }
    }
}
