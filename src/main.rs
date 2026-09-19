use axum::{
    extract::{Json, State},
    routing::{post, get},
    Router
};
use argon2::{
    Argon2,
    password_hash::PasswordHasher
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions, 
    PgPool
}
use std::env;

// Request
#[derive(Deserialize)]
struct SignupRequest {
    emial: String,
    password: String
}

// Response
#[derive(Serialize)]
struct SiginupResponse { 
    message: String
}

// health
async fn health() -> &'static str {
    "OK"
}

// signup
async fn signup(
    State(pool): Steate<PgPool>, 
    Json(req): Json<SignupRequest>
) -> Json<SignupResponse> {
    println!("Signup request for: {}", req.email);

    // create Argon2 hasher 
    let argon2 = Argon2::default();

    // hash password
    let hashedPassword = argon2::hash_password(req.password.as_bytes())
            .unwrap()
            .to_string();

    // save user to PostgreSQL
    sqlx::query(
        r#"
        INSERT INTO users (email, password_hash) 
        VALUES ($1, $2)
        "#
    )
    .bind(&req.email)
    .bind(&hashedPassword)
    .execute(&pool)
    .await
    .unwrap();
}