use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect() -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@127.0.0.1/egiraffe")
        .await?;

    Ok(pool)
}

pub(crate) async fn demo(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(42_i64)
        .fetch_one(pool)
        .await?;

    assert_eq!(row.0, 42);

    Ok(())
}
