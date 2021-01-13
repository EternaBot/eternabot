use serenity::{model::guild::Guild, prelude::Context};
use sqlx::{postgres::PgArguments, Arguments};

use crate::{models::server::Server, typemap_keys::PostgresPool};

pub async fn insert_guild(ctx: Context, guild: Guild) {
    // First aquire connection from pool.
    let data = ctx.data.read().await;
    let mut con = data
        .get::<PostgresPool>()
        .expect("Cannot fetch PostgreSQL connection pool")
        .acquire()
        .await
        .unwrap();

    // Add Guild ID as argument for filtering
    let mut fetch_args = PgArguments::default();
    fetch_args.add(guild.id.0 as i64);

    // Try fetching guild information from DB. When entry doesn't exist, it's None.
    let db_original: Option<Server> =
        match sqlx::query_as_with("SELECT * FROM servers WHERE discord_id = $1", fetch_args)
            .fetch_optional(&mut con)
            .await
        {
            Ok(val) => val,
            Err(err) => {
                // In case when something went wrong, capture error, release connection and return.
                sentry::capture_error(&err);
                con.release();
                return;
            }
        };

    // When server doesn't exist in DB, we create it.
    if db_original.is_none() {
        let mut insert_args = PgArguments::default();
        insert_args.add(guild.id.0 as i64);
        match sqlx::query_with("INSERT INTO servers (discord_id) VALUES ($1)", insert_args)
            .execute(&mut con)
            .await
        {
            Ok(_val) => {}
            Err(err) => {
                sentry::capture_error(&err);
                con.release();
                return;
            }
        };
    }
}
