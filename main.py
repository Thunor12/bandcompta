
from flask import Flask, render_template, request, redirect, url_for
import sqlite3
import datetime

from Transaction import Transaction, TransactionType

app = Flask(__name__)

db_path = "transaction.db"

def create_database():
    conn = sqlite3.connect(db_path)
    c = conn.cursor()
    Transaction.make_table_if_not_exists(c)
    conn.commit()
    conn.close()
    
@app.route('/')
def index():
    conn = sqlite3.connect(db_path)
    
    c = conn.cursor()
    c.execute('SELECT * FROM transactions')
    trs = c.fetchall()
    
    conn.close()
    
    # trss = [ Transaction(
    #     name=t[1],
    #     company=t[2],
    #     tr_type=t[3],
    #     date=datetime.datetime.strptime(t[5].split('T')[0], '%Y-%m-%d'),
    #     price_full_tax=t[6],
    #     tag=t[7],
    #     tax_amout=t[8],
    #     invoice_path=t[9]
    # ) for t in trs]
    
    tot = 0
    for t in trs:
        print(t)
        ty = t[3]
        tp = t[6]
        
        if ty == TransactionType.EXPENSE or ty == TransactionType.NDF:
            tot = tot - tp
        
        if ty == TransactionType.INCOME:
            tot = tot + tp
    
    return render_template('index.html', tasks=trs, total=tot)

@app.route('/add', methods=['POST'])
def add_task():
    tr_name = request.form['name']
    tr_co = request.form['company']
    tr_ty = TransactionType[request.form['type']] 
    # tr_ex = request.form['executed']
    print(request.form['date'])
    tr_date = datetime.datetime.strptime(request.form['date'], '%Y-%m-%dT%H:%M') 
    tr_price = request.form['price_full_tax'] 
    tr_tag = request.form['tag'] 
    tr_tax = request.form['tax_amount']
    tr_invp = request.form['invoice_path'] 
    
    
    tr = Transaction(
        name=tr_name,
        company=tr_co,
        tr_type=tr_ty,
        date=tr_date,
        price_full_tax=tr_price,
        tag=tr_tag,
        tax_amout=tr_tax,
        invoice_path=tr_invp
    )
    
    conn = sqlite3.connect(db_path)
    c = conn.cursor()
    tr.add_to_db(c)
    
    conn.commit()
    conn.close()
    
    return redirect(url_for('index'))

@app.route('/delete/<int:task_id>')
def delete_task(task_id):
    conn: sqlite3.Connection = sqlite3.connect(db_path)
    c: sqlite3.Cursor = conn.cursor()
    c.execute('DELETE FROM transactions WHERE id = ?', (task_id,))
    conn.commit()   
    conn.close()
    return redirect(url_for('index'))

if __name__ == "__main__":
        
    create_database()
    app.run(debug=True)