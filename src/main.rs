use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use yew::{html::ImplicitClone, prelude::*};

// type Date_t = DateTime<Utc>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum TransactionType {
    INCOME,
    EXPENSE,
    NDF,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Transaction {
    name: String,
    company: String,
    transaction_type: TransactionType,
    executed: bool,
    date: String,
    price_full_tax: f32,
    tag: String,
    tax_amount: f32,
    invoice_path: String,
}
impl ImplicitClone for Transaction {}

#[derive(Properties, PartialEq)]
struct TransactionTableRowProps {
    transaction: Transaction,
}

#[function_component(TransactionTableRow)]
fn transaction_table_row(
    TransactionTableRowProps { transaction }: &TransactionTableRowProps,
) -> Html {
    html! {
        <tr>
            <td> {format!("{}",   &transaction.name)              } </td>
            <td> {format!("{}",   &transaction.company)           } </td>
            <td> {format!("{:?}", &transaction.transaction_type)  } </td>
            <td> {format!("{}",   &transaction.executed)          } </td>
            <td> {format!("{}",   &transaction.date)              } </td>
            <td> {format!("{}",   &transaction.price_full_tax)    } </td>
            <td> {format!("{}",   &transaction.tag)               } </td>
            <td> {format!("{}",   &transaction.tax_amount)        } </td>
            <td> {format!("{}",   &transaction.invoice_path)      } </td>
        </tr>
    }
}

const TIME_FMT: &'static str = "%Y-%m-%d";

#[derive(Properties, PartialEq)]
struct TransactionListProps {
    transactions: Vec<Transaction>,
}

#[function_component(TransactionList)]
fn transaction_list(TransactionListProps { transactions }: &TransactionListProps) -> Html {
    let col_name = "name ";
    let col_company = "company ";
    let col_transaction_type = "transaction type ";
    let col_executed = "executed ";
    let col_date = "date ";
    let col_price_full_tax = "price full tax ";
    let col_tag = "tag ";
    let col_tax_amount = "tax amount ";
    let col_invoice_path = "invoice path";

    let h = html! {
        <table>
            <tr>
                <th> {col_name}             </th>
                <th> {col_company}          </th>
                <th> {col_transaction_type} </th>
                <th> {col_executed}         </th>
                <th> {col_date}             </th>
                <th> {col_price_full_tax}   </th>
                <th> {col_tag}              </th>
                <th> {col_tax_amount}       </th>
                <th> {col_invoice_path}     </th>
            </tr>
                {
                    transactions
                        .iter()
                        .map(|tr| {html! {<TransactionTableRow transaction={tr} />}})
                        .collect::<Html>()
                }
        </table>
    };

    h
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(App)]
fn app() -> Html {
    let trs = vec![
        Transaction {
            name: "Logo Athas 1".into(),
            company: "Moonroot".into(),
            transaction_type: TransactionType::EXPENSE,
            date: "2025-01-01".into(),
            price_full_tax: 200.0,
            tax_amount: 20.0,
            executed: true,
            tag: "Visuels".into(),
            invoice_path: "./".into(),
        },
        Transaction {
            name: "Cachet Klub".into(),
            company: "Klub".into(),
            transaction_type: TransactionType::INCOME,
            date: "2025-01-01".into(),
            price_full_tax: 50.0,
            tax_amount: 0.0,
            executed: true,
            tag: "Visuels".into(),
            invoice_path: "./".into(),
        },
    ];

    html! {
        <>
            <h1>{ "Athas Band" }</h1>
            <div>
                <h3>{"Transactions"}</h3>
                <TransactionList transactions={trs} />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
