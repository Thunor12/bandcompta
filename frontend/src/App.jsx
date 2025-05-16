import { useState, useEffect } from "react";
import { getTransactions, deleteTransaction, addTransaction } from "./data/sqlite";
import TransactionForm from "./components/TransactionForm";
import TransactionTable from "./components/TransactionTable";

function App() {
  const [transactions, setTransactions] = useState(Array());

  const fetchTransactions = () => {
    getTransactions().then(
      tr => {
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
      <TransactionTable transactions={transactions} onDelete={deleteTrans} />

      <h2>Add Transaction</h2>
      <TransactionForm onAdd={addTrans} />
    </div>
  );
}

export default App;
