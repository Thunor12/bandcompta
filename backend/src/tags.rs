use rusqlite::{Connection, Result as SqlResult};
use serde_rusqlite::{from_rows, to_params_named};

use bandcompta_shared::{NewTag, Tag, PREDEFINED_TAGS};

use crate::db::map_serde_err;

const TAGS_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
)";

pub fn init_tags(connection: &Connection) -> SqlResult<()> {
    connection.execute(TAGS_SCHEMA, [])?;
    seed_tags_if_empty(connection)?;
    Ok(())
}

fn seed_tags_if_empty(connection: &Connection) -> SqlResult<()> {
    let count: i64 = connection.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    for name in PREDEFINED_TAGS {
        insert_tag(connection, &NewTag { name: (*name).into() })?;
    }

    Ok(())
}

pub fn list_tags(connection: &Connection) -> SqlResult<Vec<Tag>> {
    let mut statement =
        connection.prepare("SELECT id, name FROM tags ORDER BY name ASC")?;
    Ok(from_rows::<Tag>(statement.query([])?)
        .filter_map(|row| row.ok())
        .collect())
}

pub fn get_tag(connection: &Connection, id: i64) -> SqlResult<Option<Tag>> {
    let mut statement = connection.prepare("SELECT id, name FROM tags WHERE id = ?1")?;
    let mut rows = from_rows::<Tag>(statement.query([id])?);
    match rows.next() {
        Some(row) => row.map(Some).map_err(map_serde_err),
        None => Ok(None),
    }
}

pub fn insert_tag(connection: &Connection, tag: &NewTag) -> SqlResult<i64> {
    let params = to_params_named(tag).map_err(map_serde_err)?;
    connection.execute(
        "INSERT INTO tags (name) VALUES (:name)",
        params.to_slice().as_slice(),
    )?;
    Ok(connection.last_insert_rowid())
}
