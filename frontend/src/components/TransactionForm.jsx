import { useState } from "react";
import { addTransaction } from "../data/sqlite";
import { transactionTemplate, transaction_label_from_value, TRANSACTION_TYPE } from "../data/transactionModel";

import { InputText } from "./InputText"
import { InputNumber } from "./InputNumber";
import { InputFile } from "./InputFile";
import { DropDown } from "./DropDown";
import { CheckBox } from "./CheckBox";

export default function TransactionForm({ onAdd }) {
  const handleSubmit = (e) => {
    e.preventDefault();

    const formData = new FormData(e.target);
    const payload = Object.fromEntries(formData);
    let t = transactionTemplate;

    t.id = 0;
    t.name = payload["name"];
    t.company = payload["company"];
    t.type = transaction_label_from_value(Number(payload["type"]));
    t.executed = (payload["executed"] === 'true') ? 1 : 0;
    t.date = payload["date"];
    t.price_full_tax = Number(payload["price_full_tax"]);
    t.tag = payload["tag"];
    t.tax_amount = Number(payload["tax_amount"]);
    // t.invoice_path = payload["invoice_path"];

    onAdd(t);
  };

  return (
    <form onSubmit={handleSubmit} className="form-grid">
      <InputText name="name" placeHolder="Name" required={true} />
      <InputText name="company" placeHolder="Company" required={false} />
      <DropDown name="type" isRequired={false} choices={TRANSACTION_TYPE} />
      <CheckBox name="executed" label="Executed" />
      <InputText name="date" placeHolder="date" required={false} />
      <InputNumber name="price_full_tax" placeHolder="Price (€)" required={true} />
      <InputText name="tag" placeHolder="Tag" required={false} />
      <InputNumber name="tax_amount" placeHolder="Tax (%)" required={true} />
      <InputFile name="invoice_path" placeHolder="Invoice Path" required={false} />
      <button type="submit">Add</button>
    </form>
  );
}
