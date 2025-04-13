import { useState } from "react";
import { addTransaction } from "../data/sqlite";
import { transactionTemplate } from "../data/transactionModel";

export default function TransactionForm({ onAdd }) {
  const [form, setForm] = useState({ ...transactionTemplate });

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setForm((f) => ({
      ...f,
      [name]: type === "checkbox" ? checked : value,
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    addTransaction(form);
    onAdd();
    setForm({ ...transactionTemplate });
  };

  return (
    <form onSubmit={handleSubmit} className="form-grid">
      <input name="name" placeholder="Name" value={form.name} onChange={handleChange} required />
      <input name="company" placeholder="Company" value={form.company} onChange={handleChange} />
      <select name="type" value={form.type} onChange={handleChange}>
        <option value="INVOICE">Invoice</option>
        <option value="EXPENSE">Expense</option>
        <option value="NDF">NDF</option>
      </select>
      <label>
        <input type="checkbox" name="executed" checked={form.executed} onChange={handleChange} />
        Executed
      </label>
      <input type="datetime-local" name="date" value={form.date} onChange={handleChange} />
      <input name="price_full_tax" placeholder="Price (€)" type="number" value={form.price_full_tax} onChange={handleChange} />
      <input name="tag" placeholder="Tag" value={form.tag} onChange={handleChange} />
      <input name="tax_amount" type="number" placeholder="Tax (%)" value={form.tax_amount} onChange={handleChange} />
      <input name="invoice_path" placeholder="Invoice Path" value={form.invoice_path} onChange={handleChange} />
      <button type="submit">Add</button>
    </form>
  );
}
