mod username;
mod password;

pub use username::Username;
pub use password::{PlainPassword, EncryptedPassword};

use rand::Rng;
use std::fmt::Display;
use anyhow::{anyhow, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, SaltString};
use crate::domain::identity::{Person, TenantId, Enablement};

/// User is the aggregate root entity representing a user in the system.
pub struct User {
    tenant_id: TenantId,
    username: Username,
    password: EncryptedPassword,
    enablement: Enablement,
    person: Person,
}