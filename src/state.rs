use sqlx::PgPool;
#[derive(Debug, Clone)]
pub struct AppState {
    pub app_name: String,
    pub pool: PgPool,
}
