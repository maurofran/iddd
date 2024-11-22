mod username;
pub use username::Username;

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

const DIGITS: &str = "0123456789";
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLS: &str = "\"`!?$?%^&*()_-+={[}]:;@'~#|\\<,>.?/";

const STRONG_THRESHOLD: usize = 20;
const VERY_STRONG_THRESHOLD: usize = 40;


/// Value object representing a plain text password.
#[derive(Debug, PartialEq, Clone)]
pub struct PlainPassword(String);

impl PlainPassword {
    /// Generates a new random password.
    pub fn generate() -> Self {
        let mut generated_password: PlainPassword = PlainPassword::default();
        let mut password = String::new();
        let mut is_strong = false;
        while !is_strong {
            let opt = rand::thread_rng().gen_range(0..4);
            match opt {
                0 => {
                    let index = rand::thread_rng().gen_range(0..LETTERS.len());
                    password.push(LETTERS.chars().nth(index).unwrap());
                }
                1 => {
                    let index = rand::thread_rng().gen_range(0..LETTERS.len());
                    password.push(LETTERS.chars().nth(index).unwrap().to_ascii_lowercase());
                }
                2 => {
                    let index = rand::thread_rng().gen_range(0..DIGITS.len());
                    password.push(DIGITS.chars().nth(index).unwrap());
                }
                3 => {
                    let index = rand::thread_rng().gen_range(0..SYMBOLS.len());
                    password.push(SYMBOLS.chars().nth(index).unwrap());
                }
                _ => {}
            }
            generated_password = password.as_str().into();
            is_strong = password.len() > 7 && generated_password.is_strong();
        }
        generated_password
    }

    /// Verify if the password meets the requirements for a very strong password.
    pub fn is_very_strong(&self) -> bool {
        self.calculate_strength() >= VERY_STRONG_THRESHOLD
    }

    /// Verify if the password meets the requirements for a strong password.
    pub fn is_strong(&self) -> bool {
        self.calculate_strength() >= STRONG_THRESHOLD
    }

    /// Verify if the password meets the requirements for a weak password.
    pub fn is_weak(&self) -> bool {
        self.calculate_strength() < STRONG_THRESHOLD
    }

    fn calculate_strength(&self) -> usize {
        let mut strength = 0;
        let length = self.0.len();
        if self.0.len() > 7 {
            strength += 10 + (length - 7);
        }
        let mut digit_count = 0;
        let mut letter_count = 0;
        let mut lower_count = 0;
        let mut upper_count = 0;
        let mut symbol_count = 0;
        for ch in self.0.chars() {
            if ch.is_alphabetic() {
                letter_count += 1;
                if ch.is_uppercase() {
                    upper_count += 1;
                } else {
                    lower_count += 1;
                }
            } else if ch.is_numeric() {
                digit_count += 1;
            } else {
                symbol_count += 1;
            }
        }
        strength += upper_count + lower_count + symbol_count;
        // bonus: letters and digits
        if letter_count >= 2 && digit_count >= 2 {
            strength += letter_count + digit_count;
        }
        strength
    }

    /// Encrypts the plain text password.
    pub fn encrypt(&self) -> Result<EncryptedPassword> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(&self.0.as_bytes(), &salt)
            .map_err(|err| anyhow!(err))?;
        return Ok(EncryptedPassword(password_hash.to_string()))
    }
}

impl Display for PlainPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "***")
    }
}

impl From<&str> for PlainPassword {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Into<String> for PlainPassword {
    fn into(self) -> String {
        self.0
    }
}

impl Default for PlainPassword {
    fn default() -> Self {
        Self("".into())
    }
}

impl AsRef<str> for PlainPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// An encrypted version of a plain text password.
#[derive(Debug, PartialEq, Clone)]
pub struct EncryptedPassword(String);

impl EncryptedPassword {
    /// Creates a new `EncryptedPassword` from the given encrypted password string.
    pub fn new(encrypted_password: &str) -> Self {
        Self(encrypted_password.to_string())
    }

    pub fn verify(&self, password: PlainPassword) -> Result<bool> {
        let parsed_hash = PasswordHash::new(&self.0)
            .map_err(|err| anyhow!(err))?;
        let argon2 = Argon2::default();
        match argon2.verify_password(&password.0.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(err) => if err == Error::Password {
                Ok(false)
            } else {
                Err(anyhow!(err))
            },
        }
    }
}

impl Display for EncryptedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "***")
    }
}

impl From<&str> for EncryptedPassword {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Into<String> for EncryptedPassword {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for EncryptedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}