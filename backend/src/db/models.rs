use super::schema::transactions;
use super::schema::transactions::dsl::transactions as transaction_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName,
)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub company: Option<String>,
    pub executed: i32,
    pub exec_date: Option<String>,
    pub price_full_tax: f32,
    pub tax_amount: Option<f32>,
    pub tag: Option<String>,
    pub invoice_path: Option<String>,
}

impl Transaction {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        transaction_dsl
            .load::<Transaction>(conn)
            .expect("Error loading transactions")
    }

    pub fn by_id(id: &i32, conn: &mut SqliteConnection) -> Option<Self> {
        if let Ok(record) = transaction_dsl.find(id).get_result::<Transaction>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_name(name_str: &str, conn: &mut SqliteConnection) -> Option<Self> {
        use super::schema::transactions::dsl::name;
        if let Ok(record) = transaction_dsl
            .filter(name.eq(name_str))
            .first::<Transaction>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_tag(tag_str: &str, conn: &mut SqliteConnection) -> Option<Self> {
        use super::schema::transactions::dsl::tag;
        if let Ok(record) = transaction_dsl
            .filter(tag.eq(tag_str))
            .first::<Transaction>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn insert(transaction: Self, conn: &mut SqliteConnection) {
        let tra = transaction;

        diesel::insert_into(transaction_dsl)
            .values(&tra)
            .execute(conn)
            .expect("Error saving new user");
    }

    pub fn create(
        name: &str,
        company: Option<&str>,
        execute: Option<i32>,
        exec_date: Option<&str>,
        price_full_tax: f32,
        tax_amount: Option<f32>,
        tag: Option<String>,
        invoice_path: Option<String>,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = Uuid::new_v4().to_u128_le() as i32;

        let new_user = Self::new_transaction_struct(
            new_id,
            name,
            company,
            execute,
            exec_date,
            price_full_tax,
            tax_amount,
            tag,
            invoice_path,
        );

        diesel::insert_into(transaction_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");

        Self::by_id(&new_id, conn)
    }

    pub fn remove_by_id(t_id: &i32, conn: &mut SqliteConnection) -> Option<()> {
        use super::schema::transactions::dsl::id;

        if let Ok(_) = diesel::delete(transaction_dsl.filter(id.eq(t_id))).execute(conn) {
            return Some(());
        }

        return None;
    }

    pub fn remove_by_name(name_str: &str, conn: &mut SqliteConnection) -> Option<()> {
        use super::schema::transactions::dsl::name;

        if let Ok(_) = diesel::delete(transaction_dsl.filter(name.eq(name_str))).execute(conn) {
            return Some(());
        }

        return None;
    }

    fn new_transaction_struct(
        id: i32,
        name: &str,
        company: Option<&str>,
        execute: Option<i32>,
        exec_date: Option<&str>,
        price_full_tax: f32,
        tax_amount: Option<f32>,
        tag: Option<String>,
        invoice_path: Option<String>,
    ) -> Self {
        Transaction {
            id: id,
            name: String::from(name),
            company: match company {
                Some(c) => Some(String::from(c)),
                None => None,
            },
            executed: match execute {
                Some(e) => (e == 1) as i32,
                None => 0,
            },
            exec_date: match exec_date {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            price_full_tax: price_full_tax,
            tax_amount: tax_amount,
            tag: tag,
            invoice_path: invoice_path,
        }
    }
}

#[cfg(test)]
mod transaction_tests;
