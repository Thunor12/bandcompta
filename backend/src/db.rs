use rusqlite::{Connection, Error as RusqliteError, Result as SqlResult};
use serde_rusqlite::{from_rows, to_params_named, Error as SerdeRusqliteError};

use crate::models::{NewTransaction, Transaction, TransactionType, TreasurySummary};

fn map_serde_err(err: SerdeRusqliteError) -> RusqliteError {
    RusqliteError::ToSqlConversionFailure(Box::new(err))
}

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    company TEXT NOT NULL,
    transaction_type INTEGER NOT NULL,
    executed INTEGER NOT NULL,
    date TEXT NOT NULL,
    price_full_tax REAL NOT NULL,
    tag TEXT NOT NULL,
    tax_amount REAL NOT NULL,
    invoice_path TEXT NOT NULL
)";

pub fn init_db(path: &str) -> SqlResult<Connection> {
    let connection = Connection::open(path)?;
    connection.execute(SCHEMA, [])?;
    seed_if_empty(&connection)?;
    Ok(connection)
}

fn seed_if_empty(connection: &Connection) -> SqlResult<()> {
    let count: i64 = connection.query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let samples = [
        NewTransaction {
            name: "Cachet festival".into(),
            company: "Les Nuits Sonores".into(),
            transaction_type: TransactionType::Income,
            executed: true,
            date: "2025-06-15".into(),
            price_full_tax: 2500.0,
            tag: "Concert".into(),
            tax_amount: 20.0,
            invoice_path: "/invoices/2025-06-festival.pdf".into(),
        },
        NewTransaction {
            name: "Location van".into(),
            company: "Europcar".into(),
            transaction_type: TransactionType::Expense,
            executed: true,
            date: "2025-06-10".into(),
            price_full_tax: 420.0,
            tag: "Transport".into(),
            tax_amount: 20.0,
            invoice_path: "/invoices/2025-06-van.pdf".into(),
        },
        NewTransaction {
            name: "Merch t-shirts".into(),
            company: "PrintShop".into(),
            transaction_type: TransactionType::Expense,
            executed: false,
            date: "2025-06-20".into(),
            price_full_tax: 680.0,
            tag: "Merch".into(),
            tax_amount: 20.0,
            invoice_path: "".into(),
        },
        NewTransaction {
            name: "Repas tournée".into(),
            company: "Resto du coin".into(),
            transaction_type: TransactionType::Ndf,
            executed: true,
            date: "2025-06-16".into(),
            price_full_tax: 85.50,
            tag: "Restauration".into(),
            tax_amount: 10.0,
            invoice_path: "/invoices/2025-06-repas.jpg".into(),
        },
        NewTransaction {
            name: "Péage autoroute".into(),
            company: "Sanef".into(),
            transaction_type: TransactionType::Ndf,
            executed: true,
            date: "2025-06-16".into(),
            price_full_tax: 32.40,
            tag: "Transport".into(),
            tax_amount: 20.0,
            invoice_path: "".into(),
        },
    ];

    for sample in samples {
        insert_transaction(connection, &sample)?;
    }

    Ok(())
}

pub fn list_transactions(connection: &Connection) -> SqlResult<Vec<Transaction>> {
    let mut statement = connection.prepare(
        "SELECT id, name, company, transaction_type, executed, date, price_full_tax, tag, tax_amount, invoice_path
         FROM transactions
         ORDER BY date DESC, id DESC",
    )?;
    Ok(from_rows::<Transaction>(statement.query([])?)
        .filter_map(|row| row.ok())
        .collect())
}

pub fn get_transaction(connection: &Connection, id: i64) -> SqlResult<Option<Transaction>> {
    let mut statement = connection.prepare(
        "SELECT id, name, company, transaction_type, executed, date, price_full_tax, tag, tax_amount, invoice_path
         FROM transactions
         WHERE id = ?1",
    )?;
    let mut rows = from_rows::<Transaction>(statement.query([id])?);
    match rows.next() {
        Some(row) => row.map(Some).map_err(map_serde_err),
        None => Ok(None),
    }
}

pub fn insert_transaction(connection: &Connection, transaction: &NewTransaction) -> SqlResult<i64> {
    let params = to_params_named(transaction).map_err(map_serde_err)?;
    connection.execute(
        "INSERT INTO transactions (name, company, transaction_type, executed, date, price_full_tax, tag, tax_amount, invoice_path)
         VALUES (:name, :company, :transaction_type, :executed, :date, :price_full_tax, :tag, :tax_amount, :invoice_path)",
        params.to_slice().as_slice(),
    )?;
    Ok(connection.last_insert_rowid())
}

pub fn treasury_summary(transactions: &[Transaction]) -> TreasurySummary {
    let mut income_total = 0.0;
    let mut expense_total = 0.0;
    let mut ndf_total = 0.0;

    for transaction in transactions {
        match transaction.transaction_type {
            TransactionType::Income => income_total += transaction.price_full_tax,
            TransactionType::Expense => expense_total += transaction.price_full_tax,
            TransactionType::Ndf => ndf_total += transaction.price_full_tax,
        }
    }

    TreasurySummary {
        income_total,
        expense_total,
        ndf_total,
        balance: income_total - expense_total - ndf_total,
        transaction_count: transactions.len(),
    }
}
