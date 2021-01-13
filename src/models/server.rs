#[derive(sqlx::FromRow)]
pub struct Server {
    id: i64,
    discord_id: i64,
    prefix: String,
}
