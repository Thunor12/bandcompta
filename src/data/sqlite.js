import initSqlJs from "sql.js";
import fs from "fs";

let db = null;

const initDB = async () => {

    const filebuffer = fs.readFileSync('test.db');

    // initSqlJs().then(SQL => {
    //     console.log("DAAAAAAAAAAAAA")
    //     db = new SQL.Database(filebuffer);
    // });

    const SQL = await initSqlJs();
    db = new SQL.Database(filebuffer);

    // Create table if not exists
    db.run(`
      CREATE TABLE IF NOT EXISTS transactions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        company TEXT,
        type TEXT,
        executed INTEGER,
        date TEXT,
        price_full_tax REAL,
        tag TEXT,
        tax_amount REAL,
        invoice_path TEXT
      );
    `);
};

const addTransaction = (transaction) => {
    const stmt = db.prepare(`
      INSERT INTO transactions (name, company, type, executed, date, price_full_tax, tag, tax_amount, invoice_path)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    `);
    stmt.run([
        transaction.name,
        transaction.company,
        transaction.type,
        transaction.executed ? 1 : 0,
        transaction.date,
        transaction.price_full_tax,
        transaction.tag,
        transaction.tax_amount,
        transaction.invoice_path,
    ]);
    stmt.free();
};

const getTransactions = () => {
    const stmt = db.prepare("SELECT * FROM transactions");
    const result = [];
    while (stmt.step()) {
        result.push(stmt.getAsObject());
    }
    stmt.free();
    return result;
};

const deleteTransaction = (id) => {
    const stmt = db.prepare("DELETE FROM transactions WHERE id = ?");
    stmt.run([id]);
    stmt.free();
};

export { initDB, addTransaction, getTransactions, deleteTransaction };