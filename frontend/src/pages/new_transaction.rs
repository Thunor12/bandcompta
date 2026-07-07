use bandcompta_shared::{ApiClient, Contact, ContactFilter, NewTransaction, TransactionType};
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::ConfirmDialog;
use crate::ui::{contact_kind_matches, input_value, parse_transaction_type, select_value, LoadState};
use crate::Route;

#[derive(Clone, PartialEq)]
struct FormState {
    name: String,
    company: String,
    transaction_type: TransactionType,
    executed: bool,
    date: String,
    price_full_tax: String,
    tag: String,
    tax_amount: String,
    invoice_path: String,
    invoice_file_name: String,
    uploading_file: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            name: String::new(),
            company: String::new(),
            transaction_type: TransactionType::Expense,
            executed: false,
            date: String::new(),
            price_full_tax: String::new(),
            tag: String::new(),
            tax_amount: "20".into(),
            invoice_path: String::new(),
            invoice_file_name: String::new(),
            uploading_file: false,
        }
    }
}

impl FormState {
    fn to_new_transaction(&self) -> Result<NewTransaction, String> {
        let price_full_tax = self
            .price_full_tax
            .replace(',', ".")
            .parse::<f32>()
            .map_err(|_| "montant TTC invalide".to_string())?;
        let tax_amount = self
            .tax_amount
            .replace(',', ".")
            .parse::<f32>()
            .map_err(|_| "TVA invalide".to_string())?;

        let transaction = NewTransaction {
            name: self.name.clone(),
            company: self.company.clone(),
            transaction_type: self.transaction_type,
            executed: self.executed,
            date: self.date.clone(),
            price_full_tax,
            tag: self.tag.clone(),
            tax_amount,
            invoice_path: self.invoice_path.clone(),
        };
        transaction.validate()?;
        Ok(transaction)
    }

    fn has_justification(&self) -> bool {
        !self.invoice_path.is_empty()
    }
}

#[function_component(NewTransactionPage)]
pub fn new_transaction_page() -> Html {
    let form = use_state(FormState::default);
    let contacts = use_state(LoadState::<Vec<Contact>>::default);
    let error = use_state(String::new);
    let success = use_state(String::new);
    let submitting = use_state(|| false);
    let show_confirm = use_state(|| false);

    {
        let contacts = contacts.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match ApiClient::default_client().list_contacts(&ContactFilter::default()).await {
                    Ok(data) => contacts.set(LoadState::Ready(data)),
                    Err(err) => contacts.set(LoadState::Error(err)),
                }
            });
            || ()
        });
    }

    let filtered_contacts = match &*contacts {
        LoadState::Ready(list) => list
            .iter()
            .filter(|contact| contact_kind_matches(contact, form.transaction_type.preferred_contact_kinds()))
            .cloned()
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    let justification_label = form.transaction_type.justification_label();

    let submit_transaction = {
        let form = form.clone();
        let error = error.clone();
        let success = success.clone();
        let submitting = submitting.clone();
        let show_confirm = show_confirm.clone();
        Callback::from(move |_| {
            error.set(String::new());
            success.set(String::new());
            submitting.set(true);
            show_confirm.set(false);

            let form = (*form).clone();
            let error = error.clone();
            let success = success.clone();
            let submitting = submitting.clone();

            spawn_local(async move {
                let payload = match form.to_new_transaction() {
                    Ok(data) => data,
                    Err(err) => {
                        error.set(err);
                        submitting.set(false);
                        return;
                    }
                };

                match ApiClient::default_client().create_transaction(&payload).await {
                    Ok(_) => success.set("Transaction enregistrée.".into()),
                    Err(err) => error.set(err),
                }
                submitting.set(false);
            });
        })
    };

    let on_submit = {
        let form = form.clone();
        let show_confirm = show_confirm.clone();
        let submit_transaction = submit_transaction.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if form.uploading_file {
                return;
            }
            if form.has_justification() {
                submit_transaction.emit(());
            } else {
                show_confirm.set(true);
            }
        })
    };

    html! {
        <>
            <header class="page-header">
                <div>
                    <h1>{ "Nouvelle transaction" }</h1>
                    <p class="subtitle">{ "Saisir une recette, une dépense ou une note de frais" }</p>
                </div>
            </header>

            if !(*error).is_empty() {
                <p class="error">{ (*error).clone() }</p>
            }
            if !(*success).is_empty() {
                <div class="success-panel">
                    <p>{ (*success).clone() }</p>
                    <Link<Route> to={Route::Explorer} classes="nav-link">{ "Voir dans l'explorateur" }</Link<Route>>
                </div>
            }

            <form class="transaction-form" onsubmit={on_submit}>
                <label class="form-field">
                    <span>{ "Type" }</span>
                    <select
                        value={match form.transaction_type {
                            TransactionType::Income => "INCOME",
                            TransactionType::Expense => "EXPENSE",
                            TransactionType::Ndf => "NDF",
                        }}
                        onchange={{
                            let form = form.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*form).clone();
                                next.transaction_type = parse_transaction_type(&select_value(&e));
                                next.company.clear();
                                form.set(next);
                            })
                        }}
                    >
                        <option value="EXPENSE">{ "Dépense" }</option>
                        <option value="INCOME">{ "Recette" }</option>
                        <option value="NDF">{ "Note de frais" }</option>
                    </select>
                </label>

                <label class="form-field">
                    <span>{ "Nom" }</span>
                    <input type="text" value={form.name.clone()} required=true oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.name = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field">
                    <span>{ "Société" }</span>
                    if filtered_contacts.is_empty() {
                        <p class="muted">{ "Aucune société correspondante. " }
                            <Link<Route> to={Route::Contacts}>{ "Ajouter une société" }</Link<Route>>
                        </p>
                    }
                    <select
                        value={form.company.clone()}
                        required={!filtered_contacts.is_empty()}
                        onchange={{
                            let form = form.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*form).clone();
                                next.company = select_value(&e);
                                form.set(next);
                            })
                        }}
                    >
                        <option value="" disabled=true selected={form.company.is_empty()}>{ "— Choisir une société —" }</option>
                        { for filtered_contacts.iter().map(|contact| html! {
                            <option value={contact.name.clone()}>{ contact.name.clone() }</option>
                        }) }
                    </select>
                </label>

                <label class="form-field">
                    <span>{ "Date" }</span>
                    <input type="date" value={form.date.clone()} required=true oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.date = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field">
                    <span>{ "Montant TTC (€)" }</span>
                    <input type="text" value={form.price_full_tax.clone()} required=true oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.price_full_tax = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field">
                    <span>{ "TVA (%)" }</span>
                    <input type="text" value={form.tax_amount.clone()} required=true oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.tax_amount = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field">
                    <span>{ "Tag" }</span>
                    <input type="text" value={form.tag.clone()} required=true placeholder="Transport, Merch…" oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.tag = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field checkbox-field">
                    <input type="checkbox" checked={form.executed} onchange={{
                        let form = form.clone();
                        Callback::from(move |e: Event| {
                            let mut next = (*form).clone();
                            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                            next.executed = input.checked();
                            form.set(next);
                        })
                    }} />
                    <span>{ if form.transaction_type == TransactionType::Income { "Reçu" } else { "Exécuté" } }</span>
                </label>

                <label class="form-field form-field-wide">
                    <span>{ justification_label }</span>
                    <input type="file" onchange={{
                        let form = form.clone();
                        let error = error.clone();
                        Callback::from(move |e: Event| {
                            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                            let Some(file) = input.files().and_then(|files| files.get(0)) else {
                                return;
                            };

                            let mut next = (*form).clone();
                            next.uploading_file = true;
                            next.invoice_path.clear();
                            next.invoice_file_name = file.name();
                            form.set(next);

                            let form = form.clone();
                            let error = error.clone();
                            spawn_local(async move {
                                match ApiClient::default_client().upload_invoice(file).await {
                                    Ok(response) => {
                                        let mut next = (*form).clone();
                                        next.invoice_path = response.path;
                                        next.uploading_file = false;
                                        form.set(next);
                                    }
                                    Err(err) => {
                                        error.set(err);
                                        let mut next = (*form).clone();
                                        next.uploading_file = false;
                                        next.invoice_file_name.clear();
                                        form.set(next);
                                    }
                                }
                            });
                        })
                    }} />
                    if form.uploading_file {
                        <span class="muted">{ "Envoi du fichier…" }</span>
                    } else if !form.invoice_file_name.is_empty() {
                        <span class="file-selected">{ format!("Fichier joint : {}", form.invoice_file_name) }</span>
                    }
                </label>

                <div class="form-actions">
                    <button type="submit" disabled={*submitting || form.uploading_file}>
                        { if *submitting { "Enregistrement…" } else { "Enregistrer" } }
                    </button>
                </div>
            </form>

            if *show_confirm {
                <ConfirmDialog
                    title="Justificatif manquant"
                    message={format!(
                        "Aucun {} n'a été joint. Voulez-vous enregistrer la transaction quand même ?",
                        justification_label.to_lowercase()
                    )}
                    confirm_label="Enregistrer sans justificatif"
                    on_confirm={{
                        let show_confirm = show_confirm.clone();
                        let submit_transaction = submit_transaction.clone();
                        Callback::from(move |_| {
                            show_confirm.set(false);
                            submit_transaction.emit(());
                        })
                    }}
                    on_cancel={{
                        let show_confirm = show_confirm.clone();
                        Callback::from(move |_| show_confirm.set(false))
                    }}
                />
            }
        </>
    }
}
