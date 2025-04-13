export const transactionTemplate = {
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
