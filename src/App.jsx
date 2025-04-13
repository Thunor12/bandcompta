import { useState, useEffect } from "react";
import { initDB, getTransactions, deleteTransaction } from "./data/sqlite";
import TransactionForm from "./components/TransactionForm";
import TransactionTable from "./components/TransactionTable";

function App() {
  const [transactions, setTransactions] = useState([]);

  const fetchTransactions = () => {
    setTransactions(getTransactions());
  };

  useEffect(() => {
    initDB().then(() => {
      fetchTransactions();
    });
  }, []);

  const addTransaction = () => {
    fetchTransactions();
  };

  const deleteTrans = (id) => {
    deleteTransaction(id);
    fetchTransactions();
  };

  return (
    <div className="container">
      <h1>Transaction Manager</h1>
      <TransactionForm onAdd={addTransaction} />
      <TransactionTable transactions={transactions} onDelete={deleteTrans} />
    </div>
  );
}

export default App;
