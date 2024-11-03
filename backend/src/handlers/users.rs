use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(
    Extension(db_pool): Extension<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    if payload.username.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username cannot be empty".to_string(),
        ));
    }
    if payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password cannot be empty".to_string(),
        ));
    }
    if payload.email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email cannot be empty".to_string()));
    }

    let password = payload.password;
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let query = r#"
        INSERT INTO users (username, email, password_hash, salt, created_at) 
        VALUES ($1, $2, $3, $4, NOW())
        "#;

    match sqlx::query(query)
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&password_hash)
        .bind(salt.as_str().as_bytes().to_vec())
        .execute(&db_pool)
        .await
    {
        Ok(_) => Ok(Json("Successfully created user")),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )),
    }
}

pub async fn login_user() -> impl IntoResponse {
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    // let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    // assert!(Argon2::default()
    //     .verify_password(password.as_bytes(), &parsed_hash)
    //     .is_ok());

    (StatusCode::OK, Json("Login successful"))
}
