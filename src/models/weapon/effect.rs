use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, PgPool};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Decode, ToSchema)]
pub struct WeaponEffect {
    pub id: i32,
    pub weapon_id: i32,
    pub level: i32,
    pub effect_type: String,
    pub effect_value: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl WeaponEffect {
    pub async fn get_by_weapon_id(
        pool: &PgPool,
        weapon_id: i32,
    ) -> Result<Vec<WeaponEffect>, sqlx::Error> {
        sqlx::query_as!(
            WeaponEffect,
            "SELECT * FROM weapon_effect WHERE weapon_id = $1",
            weapon_id
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWeaponEffect {
    pub weapon_id: i32,
    pub level: i32,
    pub effect_type: String,
    pub effect_value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWeaponEffect {
    pub weapon_id: Option<i32>,
    pub level: Option<i32>,
    pub effect_type: Option<String>,
    pub effect_value: Option<i32>,
}
