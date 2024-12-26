use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
