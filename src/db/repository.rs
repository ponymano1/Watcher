use sqlx::PgPool;

pub async fn ping_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}