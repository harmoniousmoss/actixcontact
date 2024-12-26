use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
