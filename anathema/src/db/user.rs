use rusqlite::{Connection, Result};

#[derive(Debug,Clone)]
pub struct User {
    pub id: u64, // discorse user id
    pub name: String, //?
    pub anathema: u32,
    pub credits: u32,
}

pub fn get_user_database() -> Result<Connection> {
    Connection::open("user.db")
}

pub fn create_user_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  anathema        INTEGER NOT NULL,
                  credits         INTEGER NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

pub fn insert_user(conn: &Connection, user: User) -> Result<()> {
    conn.execute(
        "INSERT INTO user (id, name, anathema, credits)
                  VALUES (?1, ?2, ?3, ?4)",
            (&user.id, &user.name, &user.anathema, &user.credits),
    )?;
    Ok(())
}

pub fn update_user(conn: &Connection, user: User) -> Result<()> {
    conn.execute(
        "UPDATE user SET name = ?2, anathema = ?3, credits = ?4 WHERE id = ?1",
        (&user.id, &user.name, &user.anathema, &user.credits),
    )?;
    Ok(())
}

pub fn delete_user(conn: &Connection, id: u64) -> Result<()> {
    conn.execute("DELETE FROM user WHERE id = ?1", &[&id])?;
    Ok(())
}

pub fn upsert_user(conn: &Connection, user: User) -> Result<()> {
    match get_user(conn, user.id) {
        Ok(_) => update_user(conn, user),
        Err(_) => insert_user(conn, user),
    }
}

pub fn get_user(conn: &Connection, id: u64) -> Result<User> {
    let mut stmt = conn.prepare("SELECT id, name, anathema, credits FROM user WHERE id = ?1")?;
    let user_iter = stmt.query_map(&[&id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            anathema: row.get(2)?,
            credits: row.get(3)?,
        })
    })?;

    for user in user_iter {
        return Ok(user?);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}
