use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("theldo.db").expect("Failed to open database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs ( id INTEGER PRIMARY KEY, url TEXT NOT NULL);",
        ).unwrap();
        conn
    }
}

// // Expose a `save_dog` endpoint on our server that takes an "image" parameter
// #[post("/api/save_dog")]
// pub async fn save_dog(image: String) -> Result<()> {
//     use std::io::Write;
//
//     // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
//     let mut file = std::fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open("dogs.txt")
//         .unwrap();
//
//     // And then write a newline to it with the image url
//     file.write_fmt(format_args!("{image}\n"));
//
//     Ok(())
// }

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[post("/api/delete_dog")]
pub async fn delete_dog(id: i64) -> Result<()> {
    DB.with(|f| f.execute("DELETE FROM dogs WHERE id=(?1)", &[&id]))?;
    Ok(())
}

#[server(endpoint = "list_dogs")]
pub async fn list_dogs() -> Result<Vec<(i64, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(dogs)
}
