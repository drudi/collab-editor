//! Authentication module — registration, login, session management, and logout.
//!
//! Provides HTTP handlers for user authentication flows:
//! - `register`: Create a new user account (POST /api/auth/register)
//! - `login`: Authenticate a user and issue a session (POST /api/auth/login)
//! - `session`: Axum extractor for validating session cookies
//! - `logout`: Clear a user's session (POST /api/auth/logout)

pub mod register;
pub mod login;
pub mod session;
pub mod logout;