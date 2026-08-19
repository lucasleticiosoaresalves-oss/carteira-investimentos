use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use uuid::Uuid;

use crate::{auth::decode_jwt, state::AppState};

pub const AUTH_COOKIE: &str = "auth_token";

pub struct AuthUser {
    pub user_id: Uuid,
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::to("/login").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let cookie_header = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .map(|c| c.trim())
            .find_map(|c| c.strip_prefix(&format!("{AUTH_COOKIE}=")))
            .ok_or(AuthRedirect)?;

        let claims = decode_jwt(token, &app_state.config.jwt_secret).map_err(|_| AuthRedirect)?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthRedirect)?;

        Ok(AuthUser { user_id })
    }
}
