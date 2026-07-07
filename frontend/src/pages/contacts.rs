use bandcompta_shared::{ApiClient, Contact, ContactFilter, ContactKind, NewContact};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::ui::{input_value, parse_contact_kind, select_value, LoadState};
use crate::Route;

#[derive(Clone, PartialEq)]
struct ContactForm {
    name: String,
    kind: ContactKind,
    email: String,
    notes: String,
}

impl Default for ContactForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            kind: ContactKind::Provider,
            email: String::new(),
            notes: String::new(),
        }
    }
}

#[function_component(ContactsPage)]
pub fn contacts_page() -> Html {
    let contacts = use_state(LoadState::<Vec<Contact>>::default);
    let form = use_state(ContactForm::default);
    let error = use_state(String::new);
    let success = use_state(String::new);
    let submitting = use_state(|| false);

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

    let reload = {
        let contacts = contacts.clone();
        Callback::from(move |_| {
            contacts.set(LoadState::Loading);
            let contacts = contacts.clone();
            spawn_local(async move {
                match ApiClient::default_client().list_contacts(&ContactFilter::default()).await {
                    Ok(data) => contacts.set(LoadState::Ready(data)),
                    Err(err) => contacts.set(LoadState::Error(err)),
                }
            });
        })
    };

    let on_submit = {
        let form = form.clone();
        let error = error.clone();
        let success = success.clone();
        let submitting = submitting.clone();
        let reload = reload.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(String::new());
            success.set(String::new());
            submitting.set(true);

            let payload = NewContact {
                name: form.name.clone(),
                kind: form.kind,
                email: form.email.clone(),
                notes: form.notes.clone(),
            };

            let error = error.clone();
            let success = success.clone();
            let submitting = submitting.clone();
            let form = form.clone();
            let reload = reload.clone();

            spawn_local(async move {
                match ApiClient::default_client().create_contact(&payload).await {
                    Ok(_) => {
                        success.set(format!("Société « {} » ajoutée.", payload.name));
                        form.set(ContactForm::default());
                        reload.emit(());
                    }
                    Err(err) => error.set(err),
                }
                submitting.set(false);
            });
        })
    };

    html! {
        <>
            <header class="page-header">
                <div>
                    <h1>{ "Sociétés & contacts" }</h1>
                    <p class="subtitle">{ "Clients, prestataires et partenaires" }</p>
                </div>
                <button onclick={reload.reform(|_| ())}>{ "Actualiser" }</button>
            </header>

            if !(*error).is_empty() {
                <p class="error">{ (*error).clone() }</p>
            }
            if !(*success).is_empty() {
                <p class="success-panel">{ (*success).clone() }</p>
            }

            <form class="transaction-form" onsubmit={on_submit}>
                <label class="form-field">
                    <span>{ "Nom de la société" }</span>
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
                    <span>{ "Type" }</span>
                    <select
                        value={match form.kind {
                            ContactKind::Client => "CLIENT",
                            ContactKind::Provider => "PROVIDER",
                            ContactKind::Both => "BOTH",
                        }}
                        onchange={{
                            let form = form.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*form).clone();
                                next.kind = parse_contact_kind(&select_value(&e));
                                form.set(next);
                            })
                        }}
                    >
                        <option value="CLIENT">{ "Client" }</option>
                        <option value="PROVIDER">{ "Prestataire" }</option>
                        <option value="BOTH">{ "Client & prestataire" }</option>
                    </select>
                </label>

                <label class="form-field">
                    <span>{ "E-mail" }</span>
                    <input type="text" value={form.email.clone()} oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.email = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field form-field-wide">
                    <span>{ "Notes" }</span>
                    <input type="text" value={form.notes.clone()} oninput={{
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form).clone();
                            next.notes = input_value(&e);
                            form.set(next);
                        })
                    }} />
                </label>

                <div class="form-actions">
                    <button type="submit" disabled={*submitting}>
                        { if *submitting { "Ajout…" } else { "Ajouter la société" } }
                    </button>
                </div>
            </form>

            <div class="table-panel">
                { match &*contacts {
                    LoadState::Loading => html! { <p class="muted">{ "Chargement…" }</p> },
                    LoadState::Error(err) => html! { <p class="error">{ err.clone() }</p> },
                    LoadState::Ready(items) if items.is_empty() => html! {
                        <p class="muted">{ "Aucune société enregistrée." }</p>
                    },
                    LoadState::Ready(items) => html! {
                        <table>
                            <thead>
                                <tr>
                                    <th>{ "Nom" }</th>
                                    <th>{ "Type" }</th>
                                    <th>{ "E-mail" }</th>
                                    <th>{ "Notes" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for items.iter().map(|contact| html! {
                                    <tr>
                                        <td>{ contact.name.clone() }</td>
                                        <td><span class={kind_badge_class(contact.kind)}>{ contact.kind.label() }</span></td>
                                        <td>{ if contact.email.is_empty() { "—".into() } else { contact.email.clone() } }</td>
                                        <td>{ if contact.notes.is_empty() { "—".into() } else { contact.notes.clone() } }</td>
                                    </tr>
                                }) }
                            </tbody>
                        </table>
                    },
                } }
            </div>

            <p class="muted">
                { "Ces sociétés sont proposées lors de la saisie d'une transaction. " }
                <Link<Route> to={Route::NewTransaction}>{ "Nouvelle transaction" }</Link<Route>>
            </p>
        </>
    }
}

fn kind_badge_class(kind: ContactKind) -> &'static str {
    match kind {
        ContactKind::Client => "badge badge-income",
        ContactKind::Provider => "badge badge-expense",
        ContactKind::Both => "badge badge-ndf",
    }
}
