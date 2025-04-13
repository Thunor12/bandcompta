import { deleteTransaction } from "../data/sqlite";

export default function TransactionTable({ transactions, onDelete }) {
    return (
        <table className="transactions-table">
            <thead>
                <tr>
                    <th>Name</th><th>Company</th><th>Type</th><th>Executed</th>
                    <th>Date</th><th>Price</th><th>Tag</th><th>Tax</th><th>Invoice</th><th>Action</th>
                </tr>
            </thead>
            <tbody>
                {transactions.map((t) => (
                    <tr key={t.id}>
                        <td>{t.name}</td>
                        <td>{t.company}</td>
                        <td>{t.type}</td>
                        <td>{t.executed ? "✓" : "✗"}</td>
                        <td>{new Date(t.date).toLocaleString()}</td>
                        <td>{t.price_full_tax} €</td>
                        <td>{t.tag}</td>
                        <td>{t.tax_amount} %</td>
                        <td>{t.invoice_path}</td>
                        <td><button onClick={() => onDelete(t.id)}>Delete</button></td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
