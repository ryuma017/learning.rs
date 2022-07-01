use argon2::password_hash::{self, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};

const PASSWORD: &str = "secret_password";

fn main() {
    let expected_password_hash = compute_password_hash(PASSWORD.into()).unwrap();

    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();

    match verify_password_hash(password, expected_password_hash) {
        Ok(_) => println!("Logged in."),
        Err(e) => println!("Failed to logged in; {}", e),
    }
}

fn compute_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
    Ok(password_hash)
}

fn verify_password_hash(
    password: String,
    expected_password_hash: String,
) -> Result<(), password_hash::Error> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &expected_password_hash)
}
