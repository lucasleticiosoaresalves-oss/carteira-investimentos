use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::assets, handlers::auth, state::AppState};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Auth
        .route("/register", get(auth::show_register).post(auth::register))
        .route("/login", get(auth::show_login).post(auth::login))
        .route("/logout", post(auth::logout))
        // Dashboard e ativos (protegidos pelo extractor AuthUser)
        .route("/dashboard", get(assets::dashboard))
        .route("/assets/new", get(assets::new_asset_form))
        .route("/assets", post(assets::create_asset))
        .route(
            "/assets/:id/edit",
            get(assets::edit_asset_form),
        )
        .route("/assets/:id", post(assets::update_asset))
        .route("/assets/:id/delete", post(assets::delete_asset))
        .with_state(state)
}
