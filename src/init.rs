use sqlx::{Executor, PgPool};

pub async fn initialize_database(db_pool: &PgPool) {
    let create_table_query = r#"
    CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        username VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    "#;

    db_pool
        .execute(create_table_query)
        .await
        .expect("Failed to initialize database");
}

pub async fn seed_admin(db_pool: &PgPool) -> Result<String, sqlx::Error> {
    use crate::utils::hash::hash_password;
    use std::env;

    let admin_username = env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let hashed_password = hash_password(&admin_password);

    let query = r#"
        INSERT INTO users (id, username, password, created_at, updated_at)
        VALUES (gen_random_uuid(), $1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT (username) DO NOTHING
    "#;

    let result = sqlx::query(query)
        .bind(&admin_username)
        .bind(&hashed_password)
        .execute(db_pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(format!(
            "✅ Admin user '{}' was created successfully.",
            admin_username
        ))
    } else {
        Ok(format!(
            "ℹ️ Admin user '{}' already exists.",
            admin_username
        ))
    }
}
