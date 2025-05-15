import initSqlJs from "sql.js";
import fs from "fs";
import { transactionTemplate } from "./transactionModel";

let db = [];

const initDB = () => {

  /*

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

    */
   if (db.length === 0) {
    db.push(transactionTemplate);
   }
};

const addTransaction = (transaction) => {
  /*
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
    */

   console.log("Adding transcation: ", transaction);
   db.push(transaction);
   console.log("Resulting transcations: ", db);
};

const getTransactions = () => {
  /*
    const stmt = db.prepare("SELECT * FROM transactions");
    const result = [];
    while (stmt.step()) {
        result.push(stmt.getAsObject());
    }
    stmt.free();
    return result;
    */
   console.log("Getting transactions");
   return Array.from(db);
};

const deleteTransaction = (id) => {
    // const stmt = db.prepare("DELETE FROM transactions WHERE id = ?");
    // stmt.run([id]);
    // stmt.free();
    console.log("ID to delete: ", id);
};

export { initDB, addTransaction, getTransactions, deleteTransaction };