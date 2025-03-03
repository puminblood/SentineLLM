use sqlx::mysql::MySqlPool;
use sqlx::Row;

fn insert_pdf(pool: &MySqlPool, encrypted_data: Vec<u8>) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO documents (pdf_content) VALUES (?)")
        .bind(encrypted_data)
        .execute(pool)
        .await?;
    Ok(())
}

fn get_pdf(pool: &MySqlPool, id: i32) -> sqlx::Result<Vec<u8>> {
    let row = sqlx::query("SELECT pdf_content FROM documents WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    let encrypted_data: Vec<u8> = row.get("pdf_content");
    Ok(encrypted_data)
}


fn main(){
    let database_url = "mysql://dorian:8jtv%jE7L2@localhost/test";
    let pool = MySqlPool::connect(database_url).await?;

    let data = [61, 76, 42, 124, 163, 227, 215, 123, 182, 233, 242, 150, 193, 42, 9, 205, 208, 44, 93, 128, 95, 62, 255, 188, 149, 139, 50, 139];

    let _ = insert_pdf(&pool, data).expect("Insertion failed");

    let data_read = get_pdf(&pool, 1).expect("Get failed");
    
}