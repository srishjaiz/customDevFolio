use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{hash_password, hash_token, new_session_token, verify_password};
use crate::http::AppState;
use crate::models::{NewAccount, NewPortfolio, NewSession, NewUser};
use crate::repos::{AccountRepo, PortfolioRepo, SessionRepo, UserRepo};
use crate::slug::{is_valid_slug, normalize_slug};

const SESSION_COOKIE: &str = "customfolio_session";
const SESSION_DAYS: i64 = 14;

pub fn router(state: Arc<AppState>) -> Router {
    let mut r = Router::new()
        .route("/health", get(health))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route("/accounts", post(create_account).get(list_accounts))
        .route(
            "/accounts/{account_id}/portfolios",
            get(list_portfolios).post(create_portfolio),
        )
        .route(
            "/accounts/{account_id}/portfolios/{slug}",
            get(get_portfolio),
        );
    r = crate::http::imports::register(r);
    r.with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "ok": true }))
}

#[derive(Debug, Deserialize)]
pub struct SignupBody {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountBody {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

pub(crate) fn api_err(status: StatusCode, msg: impl Into<String>) -> Response {
    let body = Json(serde_json::json!({ "error": msg.into() }));
    (status, body).into_response()
}

fn set_session_cookie(token: &str) -> String {
    format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        SESSION_DAYS * 24 * 3600
    )
}

fn clear_session_cookie() -> String {
    format!("{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
}

pub(crate) async fn require_user(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<crate::models::User, Response> {
    let cookie = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| api_err(StatusCode::UNAUTHORIZED, "not authenticated"))?;
    let token = cookie
        .split(';')
        .map(str::trim)
        .find_map(|p| p.strip_prefix(&format!("{SESSION_COOKIE}=")))
        .ok_or_else(|| api_err(StatusCode::UNAUTHORIZED, "not authenticated"))?;
    let token_hash = hash_token(token);
    let session = SessionRepo::new(&state.pool)
        .get_valid_by_token_hash(&token_hash, Utc::now())
        .await
        .map_err(|_| api_err(StatusCode::UNAUTHORIZED, "session expired or invalid"))?;
    UserRepo::new(&state.pool)
        .get_by_id(session.user_id)
        .await
        .map_err(|_| api_err(StatusCode::UNAUTHORIZED, "user not found"))
}

async fn signup(State(state): State<Arc<AppState>>, Json(body): Json<SignupBody>) -> Response {
    if body.password.len() < 8 {
        return api_err(
            StatusCode::BAD_REQUEST,
            "password must be at least 8 characters",
        );
    }
    let password_hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(_) => return api_err(StatusCode::INTERNAL_SERVER_ERROR, "hash failed"),
    };
    let user = match UserRepo::new(&state.pool)
        .create(&NewUser {
            email: body.email,
            password_hash: Some(password_hash),
            display_name: body.display_name,
        })
        .await
    {
        Ok(u) => u,
        Err(crate::RepoError::Conflict(m)) => return api_err(StatusCode::CONFLICT, m),
        Err(e) => return api_err(StatusCode::BAD_REQUEST, e.to_string()),
    };
    issue_session(&state, user).await
}

async fn login(State(state): State<Arc<AppState>>, Json(body): Json<LoginBody>) -> Response {
    let user = match UserRepo::new(&state.pool).get_by_email(&body.email).await {
        Ok(u) => u,
        Err(_) => return api_err(StatusCode::UNAUTHORIZED, "invalid email or password"),
    };
    let Some(ref hash) = user.password_hash else {
        return api_err(StatusCode::UNAUTHORIZED, "invalid email or password");
    };
    if !verify_password(&body.password, hash) {
        return api_err(StatusCode::UNAUTHORIZED, "invalid email or password");
    }
    issue_session(&state, user).await
}

async fn issue_session(state: &AppState, user: crate::models::User) -> Response {
    let (token, token_hash) = new_session_token();
    if let Err(e) = SessionRepo::new(&state.pool)
        .create(&NewSession {
            user_id: user.id,
            token_hash,
            expires_at: Utc::now() + Duration::days(SESSION_DAYS),
        })
        .await
    {
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }
    let view = UserView {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
    };
    (
        StatusCode::OK,
        [(header::SET_COOKIE, set_session_cookie(&token))],
        Json(view),
    )
        .into_response()
}

async fn logout() -> Response {
    (
        StatusCode::OK,
        [(header::SET_COOKIE, clear_session_cookie())],
        Json(serde_json::json!({ "ok": true })),
    )
        .into_response()
}

async fn me(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    match require_user(&state, &headers).await {
        Ok(u) => Json(UserView {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
        })
        .into_response(),
        Err(r) => r,
    }
}

async fn create_account(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateAccountBody>,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let slug = body
        .slug
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| normalize_slug(&body.name));
    if !is_valid_slug(&slug) {
        return api_err(StatusCode::BAD_REQUEST, format!("invalid slug '{slug}'"));
    }
    match AccountRepo::new(&state.pool)
        .create(&NewAccount {
            owner_user_id: user.id,
            name: body.name,
            slug,
            description: body.description,
            default_theme: None,
        })
        .await
    {
        Ok(a) => (StatusCode::CREATED, Json(a)).into_response(),
        Err(crate::RepoError::Conflict(m)) => api_err(StatusCode::CONFLICT, m),
        Err(e) => api_err(StatusCode::BAD_REQUEST, e.to_string()),
    }
}

async fn list_accounts(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    match AccountRepo::new(&state.pool).list_by_owner(user.id).await {
        Ok(list) => Json(list).into_response(),
        Err(e) => api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn ensure_account_owner(
    state: &AppState,
    user_id: Uuid,
    account_id: Uuid,
) -> Result<crate::models::Account, Response> {
    let account = AccountRepo::new(&state.pool)
        .get_by_id(account_id)
        .await
        .map_err(|_| api_err(StatusCode::NOT_FOUND, "account not found"))?;
    if account.owner_user_id != user_id {
        return Err(api_err(StatusCode::FORBIDDEN, "not account owner"));
    }
    Ok(account)
}

async fn list_portfolios(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    if let Err(r) = ensure_account_owner(&state, user.id, account_id).await {
        return r;
    }
    match PortfolioRepo::new(&state.pool)
        .list_by_account(account_id)
        .await
    {
        Ok(list) => Json(list).into_response(),
        Err(e) => api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

/// Body for creating one portfolio under an account (browser multi-create flow).
#[derive(Debug, Deserialize)]
pub struct CreatePortfolioBody {
    /// Display name of the person / portfolio owner.
    pub person_name: String,
    /// URL slug; if omitted, derived from `person_name`.
    pub slug: Option<String>,
    /// Domain preset id (frontend, backend, fullstack, …). Default: general.
    pub domain: Option<String>,
    /// Optional short title (shown under the name).
    pub title: Option<String>,
    /// Optional bio blurb.
    pub bio: Option<String>,
    /// Full portfolio config JSON; when provided, used as-is (import-style).
    pub config: Option<serde_json::Value>,
}

fn default_portfolio_config(
    person_name: &str,
    domain: &str,
    title: Option<&str>,
    bio: Option<&str>,
) -> serde_json::Value {
    let title = title.unwrap_or("Software Engineer");
    let bio = bio.unwrap_or("Portfolio generated with customFolio.");
    serde_json::json!({
        "meta": {
            "title": format!("{person_name} — {title}"),
            "description": bio,
            "siteUrl": ""
        },
        "domain": domain,
        "person": {
            "name": person_name,
            "title": title,
            "bio": bio,
            "location": "",
            "avatar": "",
            "resumeUrl": ""
        },
        "social": {},
        "greeting": {
            "headline": format!("Hi, I'm {person_name}."),
            "subheadline": title,
            "ctaLabel": "View projects",
            "ctaHref": "#projects"
        },
        "skills": { "title": "Skills", "groups": [] },
        "experience": [],
        "projects": [],
        "education": [],
        "achievements": [],
        "blog": { "enabled": false, "posts": [] },
        "contact": {
            "title": "Contact",
            "subtitle": "Get in touch",
            "email": ""
        },
        "sections": {
            "skills": true,
            "experience": true,
            "projects": true,
            "education": true,
            "achievements": true,
            "blog": false,
            "contact": true
        },
        "theme": {
            "mode": "system",
            "primary": "#2563eb"
        }
    })
}

async fn create_portfolio(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
    Json(body): Json<CreatePortfolioBody>,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    if let Err(r) = ensure_account_owner(&state, user.id, account_id).await {
        return r;
    }

    let person_name = body.person_name.trim().to_string();
    if person_name.is_empty() {
        return api_err(StatusCode::BAD_REQUEST, "person_name is required");
    }

    let slug = body
        .slug
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| normalize_slug(&person_name));
    if !is_valid_slug(&slug) {
        return api_err(StatusCode::BAD_REQUEST, format!("invalid slug '{slug}'"));
    }

    let domain = body
        .domain
        .filter(|d| !d.trim().is_empty())
        .unwrap_or_else(|| "general".into());

    let config = body.config.unwrap_or_else(|| {
        default_portfolio_config(
            &person_name,
            &domain,
            body.title
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty()),
            body.bio.as_deref().map(str::trim).filter(|s| !s.is_empty()),
        )
    });

    match PortfolioRepo::new(&state.pool)
        .create(&NewPortfolio {
            account_id,
            slug,
            domain,
            person_name,
            config,
        })
        .await
    {
        Ok(p) => (StatusCode::CREATED, Json(p)).into_response(),
        Err(crate::RepoError::Conflict(m)) => api_err(StatusCode::CONFLICT, m),
        Err(e) => api_err(StatusCode::BAD_REQUEST, e.to_string()),
    }
}

async fn get_portfolio(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((account_id, slug)): Path<(Uuid, String)>,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    if let Err(r) = ensure_account_owner(&state, user.id, account_id).await {
        return r;
    }
    match PortfolioRepo::new(&state.pool)
        .get_by_account_and_slug(account_id, &slug)
        .await
    {
        Ok(p) => Json(p).into_response(),
        Err(crate::RepoError::NotFound) => api_err(StatusCode::NOT_FOUND, "portfolio not found"),
        Err(e) => api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
