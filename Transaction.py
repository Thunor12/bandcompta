from enum import Enum
from datetime import datetime
from dataclasses import dataclass
import sqlite3

"""
| Id          | Name  | Company | Executed | Date | PriceFullTax | PaymentMethode                       | Tag                                       | TaxAmount | InvoicePath |
| ----------- | ----- | ------- | -------- | ---- | ------------ | ------------------------------------ | ----------------------------------------- | --------- | ----------- |
| (PrmaryKey) | (str) | (str)   | (bool)   | date | (€)          | (enum/str) (ex: Virement,CB,AuBlack) | (enum/str) (ex: Transport, Fourniture...) | (%)       | (path/str)  |

"""

date_fmt = '%Y-%m-%dT%H:%M:%S.%f%z'

class TransactionType(Enum):
    INCOME=0,
    EXPENSE=1,
    NDF=2

@dataclass
class Transaction:
    def __init__(self, name, company, tr_type, date, price_full_tax, tag, tax_amout, invoice_path):
        self.Name: str = name
        self.Company: str = company
        self.Type: TransactionType = tr_type
        self.Executed: bool = False
        self.Date: datetime = date
        self.PriceFullTax: float = price_full_tax
        self.Tag: str = tag
        self.TaxAmount: float = tax_amout
        self.InvoicePath: str = invoice_path
    
    @staticmethod
    def make_table_if_not_exists(con: sqlite3.Connection):
        con.execute('''CREATE TABLE IF NOT EXISTS transactions (id INTEGER PRIMARY KEY, name TEXT UNIQUE, company TEXT, type INTEGER, executed INTEGER, Date TEXT, price_full_tax REAL, tag TEXT, tax_amount REAL, invoice_path TEXT)''')
    
    @staticmethod
    def default(name) -> 'Transaction':
        return Transaction(name=name, company="", tr_type=TransactionType.INCOME, date=datetime.today(), price_full_tax=0.0, tag="", tax_amout=20.0, invoice_path="")
    
    @staticmethod
    def find_by_name(con: sqlite3.Connection, name) -> 'Transaction':
        q = 'SELECT * FROM transactions WHERE name = ?'
        con.execute(q, (name,))
        trs = con.fetchall()
        if len(trs) > 0: print(trs[0])
        
    def to_str_array(self) -> list[str]:
        l = list[str]
        l.append(self.Name)
        l.append(self.Company)
        l.append(self.Type.name)
        l.append(str(self.Executed))
        l.append(self.Date.strftime(date_fmt))
        l.append(f"{self.PriceFullTax} €")
        l.append(self.Tag)
        l.append(f"{self.TaxAmount} %")
        l.append(self.InvoicePath)
        
        return l
    
    def add_to_db(self, con: sqlite3.Connection):
        q = 'INSERT INTO transactions (name, company, type, executed, Date, price_full_tax, tag, tax_amount, invoice_path) VALUES (?,?,?,?,?,?,?,?,?)'
        print(self.Type.value[0])
        con.execute(
            q,
            (self.Name, self.Company, self.Type.value[0], int(self.Executed), self.Date.strftime(date_fmt), self.PriceFullTax, self.Tag, self.TaxAmount, self.InvoicePath,)
            )