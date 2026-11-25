//CREATE TYPE user_role AS ENUM ('Regular', 'Admin');
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(sqlx::Type)]
pub enum Role {
    Regular,
    Admin
}

#[derive(sqlx::FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    pass_hash: String,
    role: Role
}

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


use crate::paseto::Role as PasetoRole;

impl From<Role> for PasetoRole {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => PasetoRole::Admin,
            Role::Regular => PasetoRole::Regular
        }
    }
}