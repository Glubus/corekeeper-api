//! # Weapon Handlers Module
//!
//! Ce module contient les handlers pour les routes d'armes de l'API.

use axum::{extract::{Query, State, Path}, http::StatusCode, response::Json};
use serde::Deserialize;
use crate::{
    db::DatabaseManager,
    models::weapon::Weapon,
};

#[derive(Debug, Deserialize)]
pub struct WeaponListQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub category: Option<String>,
    pub weapon_type: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/weapons",
    tag = "Weapons",
    params(
        ("page" = Option<i32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i32>, Query, description = "Number of weapons per page (default: 10)"),
        ("category" = Option<String>, Query, description = "Filter by weapon category"),
        ("weapon_type" = Option<String>, Query, description = "Filter by weapon type")
    ),
    responses(
        (status = 200, description = "Weapons retrieved successfully", body = Vec<Weapon>),
        (status = 500, description = "Internal server error")
    ),
    summary = "Get list of weapons",
    description = "Retrieves a paginated list of weapons with optional filtering by category and type."
)]
pub async fn get_weapons(
    State(db): State<DatabaseManager>,
    Query(query): Query<WeaponListQuery>,
) -> Result<Json<Vec<Weapon>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    match Weapon::get_list(db.get_pool(), page, limit, query.category, query.weapon_type).await {
        Ok(weapons) => Ok(Json(weapons)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/api/weapons/{id}",
    tag = "Weapons",
    params(
        ("id" = i32, Path, description = "Weapon ID")
    ),
    responses(
        (status = 200, description = "Weapon retrieved successfully", body = Weapon),
        (status = 404, description = "Weapon not found"),
        (status = 500, description = "Internal server error")
    ),
    summary = "Get weapon by ID",
    description = "Retrieves a specific weapon by its ID including damage ranges and effects."
)]
pub async fn get_weapon_by_id(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<Weapon>, StatusCode> {
    match Weapon::get_by_id(db.get_pool(), id).await {
        Ok(weapon) => Ok(Json(weapon)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
