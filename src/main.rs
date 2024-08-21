
#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::MySqlPool::connect(&db_url).await.expect("Failed to connect to sql");
    add_result(pool).await;
    
}

async fn add_result(pool: sqlx::MySqlPool) {
    let race_id = 18;
    let position = 111;
    let bot_result = false;
    let pole = false;
    let leading_lap = false;
    let fastest_lap = false;
    let qualy_result: Option<i32> = Some(20);
    let seat_id = 9;
    let season = 3;

    let mut tx = pool.begin().await.expect("Failed to start transaction");

    sqlx::query!(
        "INSERT INTO result (position, bot_result, pole, leading_lap, fastest_lap, qualy_result, season, race_id)
            VALUES (?,?,?,?,?,?,?,?)",
        position, bot_result, pole, leading_lap, fastest_lap, qualy_result, season, race_id
    )
    .execute(&mut *tx)
    .await
    .expect("Failed to insert result");

    // Execute a separate query to get the last inserted ID
    let result = sqlx::query!(
        "SELECT LAST_INSERT_ID() AS result_id"
    )
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to fetch last insert ID");

    let result_id: u64 = result.result_id;

    sqlx::query!(
        "INSERT INTO has_result (result_id, seat_id)
            VALUES (?,?)",
        result_id, seat_id
    ).execute(&mut *tx).await.expect("Failed to insert has_result");

    tx.commit().await.expect("Failed to commit transaction");
}