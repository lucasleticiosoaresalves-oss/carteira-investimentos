use askama_axum::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use cookie::{time::Duration as CookieDuration, Cookie};

use crate::{
    auth::{generate_jwt, hash_password, verify_password},
    db,
    extractors::AUTH_COOKIE,
    models::{LoginInput, RegisterInput},
    state::AppState,
};

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    error: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
}

pub async fn show_register() -> impl IntoResponse {
    RegisterTemplate { error: None }
}

pub async fn show_login() -> impl IntoResponse {
    LoginTemplate { error: None }
}

pub async fn register(
    State(state): State<AppState>,
    Form(input): Form<RegisterInput>,
) -> Response {
    if input.name.trim().is_empty() || input.email.trim().is_empty() || input.password.len() < 6 {
        return RegisterTemplate {
            error: Some("Preencha nome, e-mail e uma senha com ao menos 6 caracteres.".into()),
        }
        .into_response();
    }

    if let Ok(Some(_)) = db::find_user_by_email(&state.pool, &input.email).await {
        return RegisterTemplate {
            error: Some("Já existe uma conta com este e-mail.".into()),
        }
        .into_response();
    }

    let hash = match hash_password(&input.password) {
        Ok(h) => h,
        Err(_) => {
            return RegisterTemplate {
                error: Some("Não foi possível processar a senha. Tente novamente.".into()),
            }
            .into_response()
        }
    };

    match db::insert_user(&state.pool, &input.name, &input.email, &hash).await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => RegisterTemplate {
            error: Some("Erro ao criar conta. Tente novamente.".into()),
        }
        .into_response(),
    }
}

pub async fn login(State(state): State<AppState>, Form(input): Form<LoginInput>) -> Response {
    let user = match db::find_user_by_email(&state.pool, &input.email).await {
        Ok(Some(u)) => u,
        _ => {
            return LoginTemplate {
                error: Some("E-mail ou senha inválidos.".into()),
            }
            .into_response()
        }
    };

    if !verify_password(&input.password, &user.password_hash) {
        return LoginTemplate {
            error: Some("E-mail ou senha inválidos.".into()),
        }
        .into_response();
    }

    let token = match generate_jwt(user.id, &state.config.jwt_secret) {
        Ok(t) => t,
        Err(_) => {
            return LoginTemplate {
                error: Some("Erro ao gerar sessão. Tente novamente.".into()),
            }
            .into_response()
        }
    };

    let cookie = Cookie::build((AUTH_COOKIE, token))
        .path("/")
        .http_only(true)
        .max_age(CookieDuration::hours(24))
        .build();

    let mut response = Redirect::to("/dashboard").into_response();
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    response
}

pub async fn logout() -> Response {
    let cookie = Cookie::build((AUTH_COOKIE, ""))
        .path("/")
        .http_only(true)
        .max_age(CookieDuration::seconds(0))
        .build();

    let mut response = Redirect::to("/login").into_response();
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    response
}
