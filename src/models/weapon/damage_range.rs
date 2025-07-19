use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, PgPool, Row};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Decode, ToSchema)]
pub struct WeaponDamageRange {
    pub id: i32,
    pub weapon_id: i32,
    pub level: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl WeaponDamageRange {
    pub async fn get_by_weapon_id(
        pool: &PgPool,
        weapon_id: i32,
    ) -> Result<Vec<WeaponDamageRange>, sqlx::Error> {
        sqlx::query_as!(
            WeaponDamageRange,
            "SELECT * FROM weapon_damage_range WHERE weapon_id = $1",
            weapon_id
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWeaponDamageRange {
    pub weapon_id: i32,
    pub level: i32,
    pub damage_min: i32,
    pub damage_max: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWeaponDamageRange {
    pub weapon_id: Option<i32>,
    pub level: Option<i32>,
    pub damage_min: Option<i32>,
    pub damage_max: Option<i32>,
}
