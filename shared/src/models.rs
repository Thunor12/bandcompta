use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Income,
    Expense,
    Ndf,
}

impl TransactionType {
    pub fn justification_label(&self) -> &'static str {
        match self {
            Self::Income => "Justificatif de recette",
            Self::Expense => "Facture",
            Self::Ndf => "Justificatif",
        }
    }

    pub fn preferred_contact_kinds(&self) -> &'static [ContactKind] {
        match self {
            Self::Income => &[ContactKind::Client, ContactKind::Both],
            Self::Expense => &[ContactKind::Provider, ContactKind::Both],
            Self::Ndf => &[ContactKind::Client, ContactKind::Provider, ContactKind::Both],
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContactKind {
    Client = 0,
    Provider = 1,
    Both = 2,
}

impl ContactKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Client => "Client",
            Self::Provider => "Prestataire",
            Self::Both => "Client & prestataire",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub kind: ContactKind,
    pub email: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewContact {
    pub name: String,
    pub kind: ContactKind,
    pub email: String,
    pub notes: String,
}

impl NewContact {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("le nom de la société est obligatoire".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewTag {
    pub name: String,
}

impl NewTag {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("le nom du tag est obligatoire".into());
        }
        Ok(())
    }
}

pub const PREDEFINED_TAGS: &[&str] = &[
    "Studio",
    "Prestation Son",
    "Prestation Lumière",
    "Prestation Musicale",
    "Transport",
    "Scénographie",
];

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactFilter {
    pub kind: Option<ContactKind>,
}

impl ContactFilter {
    pub fn to_query_string(&self) -> String {
        match self.kind {
            Some(ContactKind::Client) => "?kind=CLIENT".into(),
            Some(ContactKind::Provider) => "?kind=PROVIDER".into(),
            Some(ContactKind::Both) => "?kind=BOTH".into(),
            None => String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub company: String,
    pub transaction_type: TransactionType,
    pub executed: bool,
    pub date: String,
    pub price_full_tax: f32,
    pub tag: String,
    pub tax_amount: f32,
    pub invoice_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewTransaction {
    pub name: String,
    pub company: String,
    pub transaction_type: TransactionType,
    pub executed: bool,
    pub date: String,
    pub price_full_tax: f32,
    pub tag: String,
    pub tax_amount: f32,
    pub invoice_path: String,
}

impl NewTransaction {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("le nom est obligatoire".into());
        }
        if self.company.trim().is_empty() {
            return Err("la société est obligatoire".into());
        }
        if self.date.len() != 10 {
            return Err("la date doit être au format AAAA-MM-JJ".into());
        }
        if self.price_full_tax <= 0.0 {
            return Err("le montant TTC doit être positif".into());
        }
        if self.tag.trim().is_empty() {
            return Err("le tag est obligatoire".into());
        }
        Ok(())
    }

    pub fn has_justification(&self) -> bool {
        !self.invoice_path.trim().is_empty()
    }
}

impl From<NewTransaction> for Transaction {
    fn from(value: NewTransaction) -> Self {
        Self {
            id: 0,
            name: value.name,
            company: value.company,
            transaction_type: value.transaction_type,
            executed: value.executed,
            date: value.date,
            price_full_tax: value.price_full_tax,
            tag: value.tag,
            tax_amount: value.tax_amount,
            invoice_path: value.invoice_path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    pub path: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateFilter {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl DateFilter {
    pub fn validate(&self) -> Result<(), String> {
        let is_iso_date =
            |value: &str| value.len() == 10 && value.as_bytes().get(4) == Some(&b'-') && value.as_bytes().get(7) == Some(&b'-');

        if let Some(from) = self.from.as_ref().filter(|value| !value.is_empty()) {
            if !is_iso_date(from) {
                return Err(format!("invalid 'from' date: {from}"));
            }
        }
        if let Some(to) = self.to.as_ref().filter(|value| !value.is_empty()) {
            if !is_iso_date(to) {
                return Err(format!("invalid 'to' date: {to}"));
            }
        }
        if let (Some(from), Some(to)) = (&self.from, &self.to) {
            if !from.is_empty() && !to.is_empty() && from > to {
                return Err(format!("'from' ({from}) must be before or equal to 'to' ({to})"));
            }
        }
        Ok(())
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();
        if let Some(from) = self.from.as_ref().filter(|value| !value.is_empty()) {
            params.push(format!("from={from}"));
        }
        if let Some(to) = self.to.as_ref().filter(|value| !value.is_empty()) {
            params.push(format!("to={to}"));
        }
        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }

    pub fn from_input_value(&self) -> &str {
        self.from.as_deref().unwrap_or("")
    }

    pub fn to_input_value(&self) -> &str {
        self.to.as_deref().unwrap_or("")
    }

    pub fn set_from_input(&mut self, value: &str) {
        self.from = non_empty_option(value);
    }

    pub fn set_to_input(&mut self, value: &str) {
        self.to = non_empty_option(value);
    }

    pub fn active_label(&self) -> Option<String> {
        let from = self.from.as_deref().filter(|value| !value.is_empty());
        let to = self.to.as_deref().filter(|value| !value.is_empty());

        match (from, to) {
            (Some(from), Some(to)) => Some(format!("Période : {from} → {to}")),
            (Some(from), None) => Some(format!("À partir du {from}")),
            (None, Some(to)) => Some(format!("Jusqu'au {to}")),
            (None, None) => None,
        }
    }
}

fn non_empty_option(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TreasurySummary {
    pub income_total: f32,
    pub expense_total: f32,
    pub ndf_total: f32,
    pub balance: f32,
    pub transaction_count: usize,
}

pub fn compute_treasury_summary(transactions: &[Transaction]) -> TreasurySummary {
    let mut income_total = 0.0;
    let mut expense_total = 0.0;
    let mut ndf_total = 0.0;

    for transaction in transactions {
        match transaction.transaction_type {
            TransactionType::Income => income_total += transaction.price_full_tax,
            TransactionType::Expense => expense_total += transaction.price_full_tax,
            TransactionType::Ndf => ndf_total += transaction.price_full_tax,
        }
    }

    TreasurySummary {
        income_total,
        expense_total,
        ndf_total,
        balance: income_total - expense_total - ndf_total,
        transaction_count: transactions.len(),
    }
}
