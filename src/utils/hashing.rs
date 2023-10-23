use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

pub fn validate_hash(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    Argon2::default().verify_password(
        password.as_bytes(),
        &PasswordHash::new(password_hash).unwrap(),
    )
}
