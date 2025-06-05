use sqlx::SqlitePool;

pub async fn init_database() -> SqlitePool {
    let database_url = "sqlite:users.db";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");

    if count == 0 {
        sqlx::query("INSERT INTO users (name) VALUES (?), (?)")
            .bind("Prop")
            .bind("Bob")
            .execute(&pool)
            .await
            .expect("Failed to insert initial users");
    }

    pool
}
