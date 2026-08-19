use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Asset, CreateAssetInput, UpdateAssetInput, User};

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

// ---------- Users ----------

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn insert_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING *",
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

// ---------- Assets ----------

pub async fn list_assets_by_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Asset>> {
    sqlx::query_as::<_, Asset>(
        "SELECT * FROM assets WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_asset(pool: &PgPool, id: Uuid, user_id: Uuid) -> sqlx::Result<Option<Asset>> {
    sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn insert_asset(
    pool: &PgPool,
    user_id: Uuid,
    input: &CreateAssetInput,
) -> sqlx::Result<Asset> {
    sqlx::query_as::<_, Asset>(
        "INSERT INTO assets (user_id, name, ticker, category, quantity, unit_price)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(user_id)
    .bind(&input.name)
    .bind(&input.ticker)
    .bind(&input.category)
    .bind(input.quantity)
    .bind(input.unit_price)
    .fetch_one(pool)
    .await
}

pub async fn update_asset(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    input: &UpdateAssetInput,
) -> sqlx::Result<Option<Asset>> {
    sqlx::query_as::<_, Asset>(
        "UPDATE assets
         SET name = $1, ticker = $2, category = $3, quantity = $4,
             unit_price = $5, updated_at = now()
         WHERE id = $6 AND user_id = $7
         RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.ticker)
    .bind(&input.category)
    .bind(input.quantity)
    .bind(input.unit_price)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_asset(pool: &PgPool, id: Uuid, user_id: Uuid) -> sqlx::Result<u64> {
    let result = sqlx::query("DELETE FROM assets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

/// Soma o valor total da carteira diretamente no banco (quantity * unit_price).
/// Alternativa a somar em Rust — útil quando a lista pode ficar grande.
pub async fn portfolio_total_value(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Decimal> {
    let row: (Option<Decimal>,) = sqlx::query_as(
        "SELECT SUM(quantity * unit_price) FROM assets WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0.unwrap_or_default())
}
