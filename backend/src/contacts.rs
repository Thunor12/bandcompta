use rusqlite::{Connection, Result as SqlResult};
use serde_rusqlite::{from_rows, to_params_named};

use bandcompta_shared::{Contact, ContactFilter, ContactKind, NewContact};

use crate::db::map_serde_err;

const CONTACTS_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS contacts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    kind INTEGER NOT NULL,
    email TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT ''
)";

pub fn init_contacts(connection: &Connection) -> SqlResult<()> {
    connection.execute(CONTACTS_SCHEMA, [])?;
    seed_contacts_if_empty(connection)?;
    Ok(())
}

fn seed_contacts_if_empty(connection: &Connection) -> SqlResult<()> {
    let count: i64 = connection.query_row("SELECT COUNT(*) FROM contacts", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let samples = [
        NewContact {
            name: "Les Nuits Sonores".into(),
            kind: ContactKind::Client,
            email: "booking@nuits-sonores.fr".into(),
            notes: "Festival — client principal".into(),
        },
        NewContact {
            name: "Europcar".into(),
            kind: ContactKind::Provider,
            email: "".into(),
            notes: "Location de véhicules".into(),
        },
        NewContact {
            name: "PrintShop".into(),
            kind: ContactKind::Provider,
            email: "orders@printshop.fr".into(),
            notes: "Merch et visuels".into(),
        },
        NewContact {
            name: "Resto du coin".into(),
            kind: ContactKind::Provider,
            email: "".into(),
            notes: "".into(),
        },
        NewContact {
            name: "Sanef".into(),
            kind: ContactKind::Provider,
            email: "".into(),
            notes: "Péages autoroute".into(),
        },
    ];

    for sample in samples {
        insert_contact(connection, &sample)?;
    }

    Ok(())
}

pub fn list_contacts(connection: &Connection, filter: &ContactFilter) -> SqlResult<Vec<Contact>> {
    let sql = match filter.kind {
        Some(ContactKind::Client) => {
            "SELECT id, name, kind, email, notes FROM contacts WHERE kind IN (0, 2) ORDER BY name ASC"
        }
        Some(ContactKind::Provider) => {
            "SELECT id, name, kind, email, notes FROM contacts WHERE kind IN (1, 2) ORDER BY name ASC"
        }
        Some(ContactKind::Both) => {
            "SELECT id, name, kind, email, notes FROM contacts WHERE kind = 2 ORDER BY name ASC"
        }
        None => "SELECT id, name, kind, email, notes FROM contacts ORDER BY name ASC",
    };

    let mut statement = connection.prepare(sql)?;
    Ok(from_rows::<Contact>(statement.query([])?)
        .filter_map(|row| row.ok())
        .collect())
}

pub fn get_contact(connection: &Connection, id: i64) -> SqlResult<Option<Contact>> {
    let mut statement =
        connection.prepare("SELECT id, name, kind, email, notes FROM contacts WHERE id = ?1")?;
    let mut rows = from_rows::<Contact>(statement.query([id])?);
    match rows.next() {
        Some(row) => row.map(Some).map_err(map_serde_err),
        None => Ok(None),
    }
}

pub fn insert_contact(connection: &Connection, contact: &NewContact) -> SqlResult<i64> {
    let params = to_params_named(contact).map_err(map_serde_err)?;
    connection.execute(
        "INSERT INTO contacts (name, kind, email, notes) VALUES (:name, :kind, :email, :notes)",
        params.to_slice().as_slice(),
    )?;
    Ok(connection.last_insert_rowid())
}
