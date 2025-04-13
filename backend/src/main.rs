use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;

type Date_t = DateTime<Utc>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum TransactionType {
    INCOME,
    EXPENSE,
    NDF,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Transaction {
    name: String,
    company: String,
    transaction_type: TransactionType,
    executed: bool,
    date: String,
    price_full_tax: f32,
    tag: String,
    tax_amount: f32,
    invoice_path: String,
}

const TIME_FMT: &'static str = "%Y-%m-%d";

fn main() {
    let connection = rusqlite::Connection::open("test.db").unwrap();

    connection.execute("CREATE TABLE IF NOT EXISTS transactions (id INTEGER PRIMARY KEY, name TEXT UNIQUE, company TEXT, transaction_type INTEGER, executed INTEGER, date TEXT, price_full_tax REAL, tag TEXT, tax_amount REAL, invoice_path TEXT)", []).unwrap();

    let row1 = Transaction {
        name: "Test2".into(),
        company: "Dummy".into(),
        transaction_type: TransactionType::EXPENSE,
        executed: false,
        date: "2025-01-02".into(),
        price_full_tax: 100.0,
        tag: "Visuels".into(),
        tax_amount: 20.0,
        invoice_path: "./".into(),
    };

    // connection.execute("INSERT INTO transactions (id, name, company, transaction_type, executed, date, price_full_tax, tag, tax_amount, invoice_path) VALUES (:id, :name, :company, :transaction_type, :executed, :date, :price_full_tax, :tag, :tax_amount, :invoice_path)", to_params_named(&row1).unwrap().to_slice().as_slice()).unwrap();

    let mut statement = connection.prepare("SELECT * FROM transactions").unwrap();
    let mut res = from_rows::<Transaction>(statement.query([]).unwrap()).filter_map(|x| x.ok());

    for r in res {
        println!("{:?}", r);
    }
}
