//! # Help Routes Module
//!
//! Ce module configure les routes d'aide et de diagnostic de l'API.

use crate::{db::DatabaseManager, handlers::weapon};
use axum::{Router, routing::get};

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/weapons", get(weapon::get_weapons))
        .route("/weapons/{id}", get(weapon::get_weapon_by_id))
}
