import { useState, useEffect } from "react";
import { getTransactions, deleteTransaction, addTransaction } from "./data/sqlite";
import TransactionForm from "./components/TransactionForm";
import TransactionTable from "./components/TransactionTable";

function App() {
  const [transactions, setTransactions] = useState(Array());

  const fetchTransactions = () => {
    getTransactions().then(
      tr => {
        // Looks like we need to have transactions and setTransactions in the same scope to make the update work
        let trr = transactions;
        trr = tr;
        setTransactions(trr);
      }
    )
  };

  useEffect(() => {
    fetchTransactions();
  }, []);

  const addTrans = (t) => {
    addTransaction(t).then(() => {
      fetchTransactions();
    });
  };

  const deleteTrans = (tr) => {
    deleteTransaction(tr).then(() => {
      fetchTransactions();
    }
    );
  };

  return (
    <div className="container">
      <h1>Transaction Manager</h1>

      <h2>Transaction List</h2>
      <div className="card">
        <TransactionTable transactions={transactions} onDelete={deleteTrans} />
      </div>

      <h2>Add Transaction</h2>
      <div className="card">
        <TransactionForm onAdd={addTrans} />
      </div>

      <footer>
        © {new Date().getFullYear()} MyApp. All rights reserved.
      </footer>

    </div>
  );
}

export default App;
