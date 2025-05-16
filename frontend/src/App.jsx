import { useState, useEffect } from "react";
import { initDB, getTransactions, deleteTransaction, addTransaction } from "./data/sqlite";
import TransactionForm from "./components/TransactionForm";
import TransactionTable from "./components/TransactionTable";
import { transactionTemplate } from "./data/transactionModel";

function App() {
  const [transactions, setTransactions] = useState(Array());

  const fetchTransactions = () => {
    getTransactions().then(re => setTransactions(re))
  };

  useEffect(() => {
    initDB();
    fetchTransactions();
  }, []);

  const addTrans = (t) => {
    addTransaction(t).then(_ => fetchTransactions());
  };

  const deleteTrans = (id) => {
    deleteTransaction(id);
    fetchTransactions();
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
