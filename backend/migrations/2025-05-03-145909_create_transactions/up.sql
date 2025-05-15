CREATE TABLE IF NOT EXISTS transactions (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  company TEXT,
  executed INT NOT NULL,
  exec_date TEXT,
  price_full_tax REAL NOT NULL,
  tax_amount REAL,
  tag TEXT,
  invoice_path TEXT
);
