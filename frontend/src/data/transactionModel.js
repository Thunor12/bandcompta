
export const transaction_type = {
    INVOICE: 0,
    EXPENSE: 1,
    NDF: 2,
}

export const TRANSACTION_TYPE = [
    {
        "value": transaction_type.INVOICE,
        "label": "Invoice",
    },
    {
        "value": transaction_type.EXPENSE,
        "label": "Expense",
    },
    {
        "value": transaction_type.NDF,
        "label": "Note de frais",
    },
];


export const transactionTemplate = {
    id: 0,
    name: "",
    company: "",
    type: "INVOICE",
    executed: false,
    date: new Date().toISOString().slice(0, 16),
    price_full_tax: "",
    tag: "",
    tax_amount: 20.0,
    invoice_path: ""
};
