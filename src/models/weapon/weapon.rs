use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, PgPool, types::BigDecimal};
use crate::models::weapon::{CreateWeaponDamageRange, CreateWeaponEffect, WeaponDamageRange, WeaponEffect};
use bigdecimal::ToPrimitive;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Weapon {
    pub id: i32,
    pub name: String,
    pub weapon_category: String,
    pub weapon_type: String,
    pub level: i32,
    pub durability: i32,
    pub attack_speed: f64,
    pub rarity: Option<String>,
    pub crafting_exp: i32,
    pub description: Option<String>,
    pub sell_price: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub damage_ranges: Vec<WeaponDamageRange>,
    pub effects: Vec<WeaponEffect>,
}
impl From<WeaponRow> for Weapon {
    fn from(row: WeaponRow) -> Self {
        Weapon {
            id: row.id,
            name: row.name,
            weapon_category: row.weapon_category,
            weapon_type: row.weapon_type,
            level: row.level,
            durability: row.durability,
            attack_speed: row.attack_speed.to_f64().unwrap(),
            rarity: row.rarity,
            crafting_exp: row.crafting_exp,
            description: row.description,
            sell_price: row.sell_price,
            created_at: row.created_at,
            updated_at: row.updated_at,
            damage_ranges: Vec::new(),
            effects: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWeapon {
    pub name: String,
    pub weapon_category: String,
    pub weapon_type: String,
    pub level: i32,
    pub durability: i32,
    pub attack_speed: f64,
    pub rarity: Option<String>,
    pub crafting_exp: i32,
    pub description: Option<String>,
    pub sell_price: Option<i32>,
    pub damage_ranges: Vec<CreateWeaponDamageRange>,
    pub effects: Vec<CreateWeaponEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWeapon {
    pub name: Option<String>,
    pub weapon_category: Option<String>,
    pub weapon_type: Option<String>,
    pub level: Option<i32>,
    pub durability: Option<i32>,
    pub attack_speed: Option<f32>,
    pub rarity: Option<String>,
    pub crafting_exp: Option<i32>,
    pub description: Option<String>,
    pub sell_price: Option<i32>,
}


#[derive(Debug, Clone,  FromRow, Decode)]
pub struct WeaponRow
    {
        pub id: i32,
        pub name: String,
        pub weapon_category: String,
        pub weapon_type: String,
        pub level: i32,
        pub durability: i32,
        pub attack_speed: BigDecimal,
        pub rarity: Option<String>,
        pub crafting_exp: i32,
        pub description: Option<String>,
        pub sell_price: Option<i32>,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }

impl Weapon {
    pub async fn get_list(
        pool: &PgPool,
        page: i32,
        limit: i32,
        category: Option<String>,
        weapon_type: Option<String>,
    ) -> Result<Vec<Weapon>, sqlx::Error> {
        let offset = (page - 1) * limit;
        
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, name, weapon_category, weapon_type, level, durability, attack_speed, rarity, crafting_exp, description, sell_price, created_at, updated_at FROM weapon WHERE 1=1"
        );
        
        if let Some(cat) = category {
            query.push(" AND weapon_category = ");
            query.push_bind(cat);
        }
        
        if let Some(wtype) = weapon_type {
            query.push(" AND weapon_type = ");
            query.push_bind(wtype);
        }
        
        query.push(" ORDER BY id LIMIT ");
        query.push_bind(limit);
        query.push(" OFFSET ");
        query.push_bind(offset);
        
        let result = query
            .build_query_as::<WeaponRow>()
            .fetch_all(pool)
            .await?;

        let mut weapons = Vec::new();
        for row in result {
            let mut weapon = Weapon::from(row);
            weapon.damage_ranges = WeaponDamageRange::get_by_weapon_id(pool, weapon.id).await?;
            weapon.effects = WeaponEffect::get_by_weapon_id(pool, weapon.id).await?;
            weapons.push(weapon);
        }

        Ok(weapons)
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Weapon, sqlx::Error> {
        let weapon = sqlx::query_as!(
            WeaponRow,
            "SELECT * FROM weapon WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(weapon) = weapon {
            let mut weapon = Weapon::from(weapon);
            weapon.damage_ranges = WeaponDamageRange::get_by_weapon_id(pool, weapon.id).await?;
            weapon.effects = WeaponEffect::get_by_weapon_id(pool, weapon.id).await?;
            Ok(weapon)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
} 