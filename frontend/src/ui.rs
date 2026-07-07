use bandcompta_shared::TransactionType;
use web_sys::wasm_bindgen::JsCast;
use web_sys::Event;

use bandcompta_shared::ContactKind;

pub fn format_euro(value: f32) -> String {
    format!("{value:.2} €")
}

pub trait TransactionTypeUi {
    fn label(&self) -> &'static str;
    fn css_class(&self) -> &'static str;
}

impl TransactionTypeUi for TransactionType {
    fn label(&self) -> &'static str {
        match self {
            TransactionType::Income => "Recette",
            TransactionType::Expense => "Dépense",
            TransactionType::Ndf => "Note de frais",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            TransactionType::Income => "badge badge-income",
            TransactionType::Expense => "badge badge-expense",
            TransactionType::Ndf => "badge badge-ndf",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum LoadState<T> {
    Loading,
    Ready(T),
    Error(String),
}

impl<T> Default for LoadState<T> {
    fn default() -> Self {
        Self::Loading
    }
}

pub fn input_value(event: &Event) -> String {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

pub fn select_value(event: &Event) -> String {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

pub fn parse_contact_kind(value: &str) -> ContactKind {
    match value {
        "CLIENT" => ContactKind::Client,
        "PROVIDER" => ContactKind::Provider,
        "BOTH" => ContactKind::Both,
        _ => ContactKind::Provider,
    }
}

pub fn contact_kind_matches(contact: &bandcompta_shared::Contact, kinds: &[ContactKind]) -> bool {
    kinds.contains(&contact.kind) || contact.kind == ContactKind::Both
}

pub fn parse_transaction_type(value: &str) -> TransactionType {
    match value {
        "INCOME" => TransactionType::Income,
        "NDF" => TransactionType::Ndf,
        _ => TransactionType::Expense,
    }
}
