use bandcompta_shared::{ApiClient, DateFilter, Transaction, TransactionType, TreasurySummary};
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum LoadState<T> {
    Loading,
    Ready(T),
    Error(String),
}

impl<T> Default for LoadState<T> {
    fn default() -> Self {
        Self::Loading
    }
}

trait TransactionTypeUi {
    fn label(&self) -> &'static str;
    fn css_class(&self) -> &'static str;
}

impl TransactionTypeUi for TransactionType {
    fn label(&self) -> &'static str {
        match self {
            Self::Income => "Recette",
            Self::Expense => "Dépense",
            Self::Ndf => "Note de frais",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            Self::Income => "badge badge-income",
            Self::Expense => "badge badge-expense",
            Self::Ndf => "badge badge-ndf",
        }
    }
}

fn format_euro(value: f32) -> String {
    format!("{value:.2} €")
}

fn load_data(
    filter: DateFilter,
    transactions: UseStateHandle<LoadState<Vec<Transaction>>>,
    summary: UseStateHandle<LoadState<TreasurySummary>>,
) {
    transactions.set(LoadState::Loading);
    summary.set(LoadState::Loading);

    let transactions_for_list = transactions.clone();
    let filter_for_list = filter.clone();
    spawn_local(async move {
        let client = ApiClient::default_client();
        match client.list_transactions(&filter_for_list).await {
            Ok(data) => transactions_for_list.set(LoadState::Ready(data)),
            Err(err) => transactions_for_list.set(LoadState::Error(err)),
        }
    });

    let summary_for_fetch = summary.clone();
    spawn_local(async move {
        let client = ApiClient::default_client();
        match client.summary(&filter).await {
            Ok(data) => summary_for_fetch.set(LoadState::Ready(data)),
            Err(err) => summary_for_fetch.set(LoadState::Error(err)),
        }
    });
}

#[derive(Properties, PartialEq)]
struct SummaryCardsProps {
    summary: TreasurySummary,
}

#[function_component(SummaryCards)]
fn summary_cards(SummaryCardsProps { summary }: &SummaryCardsProps) -> Html {
    html! {
        <div class="summary-grid">
            <div class="summary-card">
                <span class="summary-label">{ "Recettes" }</span>
                <span class="summary-value income">{ format_euro(summary.income_total) }</span>
            </div>
            <div class="summary-card">
                <span class="summary-label">{ "Dépenses" }</span>
                <span class="summary-value expense">{ format_euro(summary.expense_total) }</span>
            </div>
            <div class="summary-card">
                <span class="summary-label">{ "Notes de frais" }</span>
                <span class="summary-value ndf">{ format_euro(summary.ndf_total) }</span>
            </div>
            <div class="summary-card highlight">
                <span class="summary-label">{ "Trésorerie" }</span>
                <span class="summary-value">{ format_euro(summary.balance) }</span>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DateFilterBarProps {
    filter: DateFilter,
    on_change: Callback<DateFilter>,
    on_clear: Callback<()>,
}

#[function_component(DateFilterBar)]
fn date_filter_bar(DateFilterBarProps { filter, on_change, on_clear }: &DateFilterBarProps) -> Html {
    let on_from_change = {
        let filter = filter.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let mut next = filter.clone();
            next.set_from_input(&input.value());
            on_change.emit(next);
        })
    };

    let on_to_change = {
        let filter = filter.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let mut next = filter.clone();
            next.set_to_input(&input.value());
            on_change.emit(next);
        })
    };

    html! {
        <div class="filter-bar">
            <label class="filter-field">
                <span>{ "Du" }</span>
                <input type="date" value={filter.from.clone().unwrap_or_default()} onchange={on_from_change} />
            </label>
            <label class="filter-field">
                <span>{ "Au" }</span>
                <input type="date" value={filter.to.clone().unwrap_or_default()} onchange={on_to_change} />
            </label>
            <button type="button" class="button-secondary" onclick={on_clear.reform(|_| ())}>{ "Effacer" }</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TransactionTableProps {
    transactions: Vec<Transaction>,
    selected_id: Option<i64>,
    on_select: Callback<i64>,
}

#[function_component(TransactionTable)]
fn transaction_table(
    TransactionTableProps {
        transactions,
        selected_id,
        on_select,
    }: &TransactionTableProps,
) -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <th>{ "Type" }</th>
                    <th>{ "Nom" }</th>
                    <th>{ "Société" }</th>
                    <th>{ "Date" }</th>
                    <th>{ "Montant TTC" }</th>
                    <th>{ "Tag" }</th>
                    <th>{ "Statut" }</th>
                </tr>
            </thead>
            <tbody>
                { for transactions.iter().map(|transaction| {
                    let row_class = if *selected_id == Some(transaction.id) {
                        "selected-row"
                    } else {
                        ""
                    };
                    let on_select = {
                        let on_select = on_select.clone();
                        let id = transaction.id;
                        Callback::from(move |_| on_select.emit(id))
                    };

                    html! {
                        <tr class={row_class} onclick={on_select}>
                            <td><span class={transaction.transaction_type.css_class()}>{ transaction.transaction_type.label() }</span></td>
                            <td>{ transaction.name.clone() }</td>
                            <td>{ transaction.company.clone() }</td>
                            <td>{ transaction.date.clone() }</td>
                            <td>{ format_euro(transaction.price_full_tax) }</td>
                            <td>{ transaction.tag.clone() }</td>
                            <td>{ if transaction.executed { "Exécuté" } else { "En attente" } }</td>
                        </tr>
                    }
                }) }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
struct TransactionDetailsProps {
    transaction: Transaction,
}

#[function_component(TransactionDetails)]
fn transaction_details(TransactionDetailsProps { transaction }: &TransactionDetailsProps) -> Html {
    html! {
        <div class="details-panel">
            <h2>{ "Détails" }</h2>
            <dl>
                <dt>{ "ID" }</dt>
                <dd>{ transaction.id }</dd>
                <dt>{ "Type" }</dt>
                <dd><span class={transaction.transaction_type.css_class()}>{ transaction.transaction_type.label() }</span></dd>
                <dt>{ "Nom" }</dt>
                <dd>{ transaction.name.clone() }</dd>
                <dt>{ "Société" }</dt>
                <dd>{ transaction.company.clone() }</dd>
                <dt>{ "Date" }</dt>
                <dd>{ transaction.date.clone() }</dd>
                <dt>{ "Montant TTC" }</dt>
                <dd>{ format_euro(transaction.price_full_tax) }</dd>
                <dt>{ "TVA (%)" }</dt>
                <dd>{ format!("{} %", transaction.tax_amount) }</dd>
                <dt>{ "Tag" }</dt>
                <dd>{ transaction.tag.clone() }</dd>
                <dt>{ "Statut" }</dt>
                <dd>{ if transaction.executed { "Exécuté" } else { "En attente" } }</dd>
                <dt>{ "Justificatif" }</dt>
                <dd>{
                    if transaction.invoice_path.is_empty() {
                        html! { <span class="muted">{ "Aucun" }</span> }
                    } else {
                        html! { <code>{ transaction.invoice_path.clone() }</code> }
                    }
                }</dd>
            </dl>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let transactions = use_state(LoadState::<Vec<Transaction>>::default);
    let summary = use_state(LoadState::<TreasurySummary>::default);
    let selected_id = use_state(|| None::<i64>);
    let date_filter = use_state(DateFilter::default);

    {
        let transactions = transactions.clone();
        let summary = summary.clone();
        let date_filter = (*date_filter).clone();
        use_effect_with(date_filter, move |filter| {
            load_data(filter.clone(), transactions, summary);
            || ()
        });
    }

    let on_refresh = {
        let transactions = transactions.clone();
        let summary = summary.clone();
        let date_filter = (*date_filter).clone();
        Callback::from(move |_| {
            load_data(
                date_filter.clone(),
                transactions.clone(),
                summary.clone(),
            )
        })
    };

    let on_filter_change = {
        let date_filter = date_filter.clone();
        let selected_id = selected_id.clone();
        Callback::from(move |next: DateFilter| {
            selected_id.set(None);
            date_filter.set(next);
        })
    };

    let on_filter_clear = {
        let date_filter = date_filter.clone();
        Callback::from(move |_| date_filter.set(DateFilter::default()))
    };

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: i64| selected_id.set(Some(id)))
    };

    let selected_transaction = match &*transactions {
        LoadState::Ready(items) => selected_id.and_then(|id| items.iter().find(|t| t.id == id).cloned()),
        _ => None,
    };

    let active_filter_label = (*date_filter).active_label();

    html! {
        <>
            <header class="page-header">
                <div>
                    <h1>{ "Bandcompta" }</h1>
                    <p class="subtitle">{ "Explorateur de transactions" }</p>
                </div>
                <button onclick={on_refresh}>{ "Actualiser" }</button>
            </header>

            <DateFilterBar
                filter={(*date_filter).clone()}
                on_change={on_filter_change}
                on_clear={on_filter_clear}
            />

            if let Some(label) = active_filter_label {
                <p class="filter-active">{ label }</p>
            }

            { match &*summary {
                LoadState::Ready(data) => html! { <SummaryCards summary={data.clone()} /> },
                LoadState::Loading => html! { <p class="muted">{ "Chargement du résumé…" }</p> },
                LoadState::Error(err) => html! { <p class="error">{ format!("Erreur résumé : {err}") }</p> },
            } }

            <div class="explorer-layout">
                <div class="table-panel">
                    { match &*transactions {
                        LoadState::Loading => html! { <p class="muted">{ "Chargement des transactions…" }</p> },
                        LoadState::Error(err) => html! {
                            <div class="error-panel">
                                <p>{ format!("Impossible de joindre l'API : {err}") }</p>
                                <p class="muted">{ "Lancez le backend avec `make run-backend`." }</p>
                            </div>
                        },
                        LoadState::Ready(items) if items.is_empty() => html! {
                            <p class="muted">{ "Aucune transaction pour cette période." }</p>
                        },
                        LoadState::Ready(items) => html! {
                            <TransactionTable
                                transactions={items.clone()}
                                selected_id={*selected_id}
                                on_select={on_select}
                            />
                        },
                    } }
                </div>

                { if let Some(transaction) = selected_transaction {
                    html! { <TransactionDetails transaction={transaction} /> }
                } else {
                    html! {
                        <div class="details-panel empty">
                            <p class="muted">{ "Sélectionnez une transaction pour voir les détails." }</p>
                        </div>
                    }
                } }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
