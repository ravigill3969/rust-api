use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password_hash: &str, password: &str) -> bool {
    println!("{}  {}", password_hash , password);
    verify(password, password_hash).unwrap_or(false)
}
