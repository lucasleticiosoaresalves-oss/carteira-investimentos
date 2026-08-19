use askama_axum::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use uuid::Uuid;

use crate::{
    db,
    extractors::AuthUser,
    models::{AssetView, CreateAssetInput, UpdateAssetInput},
    state::AppState,
};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    assets: Vec<AssetView>,
    total_value: String,
    asset_count: usize,
}

#[derive(Template)]
#[template(path = "asset_form.html")]
struct AssetFormTemplate {
    title: String,
    action: String,
    error: Option<String>,
    asset: Option<AssetView>,
}

pub async fn dashboard(State(state): State<AppState>, user: AuthUser) -> Response {
    let assets = match db::list_assets_by_user(&state.pool, user.user_id).await {
        Ok(a) => a,
        Err(_) => return Redirect::to("/login").into_response(),
    };

    // Melhoria: valor total da carteira, calculado em Rust a partir
    // dos ativos já carregados (quantidade * preço unitário de cada um).
    let total: rust_decimal::Decimal = assets.iter().map(|a| a.total_value()).sum();

    let asset_count = assets.len();
    let views: Vec<AssetView> = assets.iter().map(AssetView::from).collect();

    DashboardTemplate {
        assets: views,
        total_value: format!("{:.2}", total),
        asset_count,
    }
    .into_response()
}

pub async fn new_asset_form() -> impl IntoResponse {
    AssetFormTemplate {
        title: "Novo ativo".into(),
        action: "/assets".into(),
        error: None,
        asset: None,
    }
}

pub async fn create_asset(
    State(state): State<AppState>,
    user: AuthUser,
    Form(input): Form<CreateAssetInput>,
) -> Response {
    if input.name.trim().is_empty() || input.ticker.trim().is_empty() {
        return AssetFormTemplate {
            title: "Novo ativo".into(),
            action: "/assets".into(),
            error: Some("Nome e ticker são obrigatórios.".into()),
            asset: None,
        }
        .into_response();
    }

    if input.quantity < rust_decimal::Decimal::ZERO || input.unit_price < rust_decimal::Decimal::ZERO {
        return AssetFormTemplate {
            title: "Novo ativo".into(),
            action: "/assets".into(),
            error: Some("Quantidade e preço não podem ser negativos.".into()),
            asset: None,
        }
        .into_response();
    }

    match db::insert_asset(&state.pool, user.user_id, &input).await {
        Ok(_) => Redirect::to("/dashboard").into_response(),
        Err(_) => AssetFormTemplate {
            title: "Novo ativo".into(),
            action: "/assets".into(),
            error: Some("Erro ao salvar o ativo. Tente novamente.".into()),
            asset: None,
        }
        .into_response(),
    }
}

pub async fn edit_asset_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Response {
    let asset = match db::find_asset(&state.pool, id, user.user_id).await {
        Ok(Some(a)) => a,
        _ => return Redirect::to("/dashboard").into_response(),
    };

    AssetFormTemplate {
        title: "Editar ativo".into(),
        action: format!("/assets/{id}"),
        error: None,
        asset: Some(AssetView::from(&asset)),
    }
    .into_response()
}

pub async fn update_asset(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Form(input): Form<UpdateAssetInput>,
) -> Response {
    if input.quantity < rust_decimal::Decimal::ZERO || input.unit_price < rust_decimal::Decimal::ZERO {
        return Redirect::to(&format!("/assets/{id}/edit")).into_response();
    }

    match db::update_asset(&state.pool, id, user.user_id, &input).await {
        Ok(Some(_)) => Redirect::to("/dashboard").into_response(),
        _ => Redirect::to("/dashboard").into_response(),
    }
}

pub async fn delete_asset(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Response {
    let _ = db::delete_asset(&state.pool, id, user.user_id).await;
    Redirect::to("/dashboard").into_response()
}
