use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Asset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub ticker: String,
    pub category: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Asset {
    pub fn total_value(&self) -> Decimal {
        self.quantity * self.unit_price
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAssetInput {
    pub name: String,
    pub ticker: String,
    pub category: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetInput {
    pub name: String,
    pub ticker: String,
    pub category: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
}

/// Representação já formatada de um ativo para exibir no template
/// (evita fazer contas dentro do HTML).
pub struct AssetView {
    pub id: Uuid,
    pub name: String,
    pub ticker: String,
    pub category: String,
    pub quantity: String,
    pub unit_price: String,
    pub total_value: String,
}

impl From<&Asset> for AssetView {
    fn from(a: &Asset) -> Self {
        Self {
            id: a.id,
            name: a.name.clone(),
            ticker: a.ticker.clone(),
            category: a.category.clone(),
            quantity: format!("{:.4}", a.quantity),
            unit_price: format!("{:.2}", a.unit_price),
            total_value: format!("{:.2}", a.total_value()),
        }
    }
}
