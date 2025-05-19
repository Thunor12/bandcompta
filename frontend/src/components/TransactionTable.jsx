import { useState } from "react";
import { deleteTransaction } from "../data/sqlite";

export default function TransactionTable({ transactions, onDelete }) {
  const [sortField, setSortField] = useState("date");
  const [sortAsc, setSortAsc] = useState(false);

  const sorted = [...transactions].sort((a, b) => {
    const aVal = a[sortField];
    const bVal = b[sortField];

    if (typeof aVal === "string") return sortAsc ? aVal.localeCompare(bVal) : bVal.localeCompare(aVal);
    if (typeof aVal === "number") return sortAsc ? aVal - bVal : bVal - aVal;
    if (aVal instanceof Date || sortField === "date") return sortAsc ? new Date(aVal) - new Date(bVal) : new Date(bVal) - new Date(aVal);
    return 0;
  });

  const toggleSort = (field) => {
    if (field === sortField) {
      setSortAsc(!sortAsc);
    } else {
      setSortField(field);
      setSortAsc(true);
    }
  };

  const renderSortIcon = (field) => {
    if (field !== sortField) return "↕";
    return sortAsc ? "↑" : "↓";
  };

  return (
    <table className="transactions-table">
      <thead>
        <tr>
          <th onClick={() => toggleSort("name")}>Name {renderSortIcon("name")}</th>
          <th onClick={() => toggleSort("company")}>Company {renderSortIcon("company")}</th>
          <th onClick={() => toggleSort("type")}>Type {renderSortIcon("type")}</th>
          <th onClick={() => toggleSort("executed")}>Executed {renderSortIcon("executed")}</th>
          <th onClick={() => toggleSort("date")}>Date {renderSortIcon("date")}</th>
          <th onClick={() => toggleSort("price_full_tax")}>Price {renderSortIcon("price_full_tax")}</th>
          <th onClick={() => toggleSort("tag")}>Tag {renderSortIcon("tag")}</th>
          <th onClick={() => toggleSort("tax_amount")}>Tax {renderSortIcon("tax_amount")}</th>
          <th>Invoice</th>
          <th>Action</th>
        </tr>
      </thead>
      <tbody>
        {sorted.map((t) => (
          <tr key={t.name}>
            <td>{t.name}</td>
            <td>{t.company}</td>
            <td>{t.type}</td>
            <td>{t.executed ? "✓" : "✗"}</td>
            <td>{new Date(t.date).toLocaleString()}</td>
            <td>{t.price_full_tax} €</td>
            <td>{t.tag}</td>
            <td>{t.tax_amount} %</td>
            <td title={t.invoice_path} style={{ maxWidth: "150px", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
              {t.invoice_path}
            </td>
            <td><button onClick={() => onDelete(t)}>Delete</button></td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
