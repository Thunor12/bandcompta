
use std::i32;

use crate::db::{establish_connection, models::Transaction};

#[test]
fn create_transaction_with_name_and_compagny() {
    let mut conn: diesel::SqliteConnection = establish_connection();

    let name = "testname";
    let company = Some("TestCo");

    let tr: Transaction = Transaction::create(
        name, company, None, None, 20.0, None, None, None, &mut conn,
    )
    .expect("Failed to create trans");

    assert_eq!(tr.name.as_str(), name);
    assert_eq!(tr.company.unwrap().as_str(), company.unwrap());
}

#[test]
fn delete_transaction_by_id() {

    let mut conn: diesel::SqliteConnection = establish_connection();

    let name = "testname";
    let company = Some("TestCo");

    let tr: Transaction = Transaction::create(  
        name, company, None, None, 20.0, None, None, None, &mut conn,
    )
    .expect("Failed to create trans");

    assert!(Transaction::remove_by_id(&tr.id, &mut conn).is_some());

    assert_eq!(Transaction::list(&mut conn).len(), 0);
}


// TODO it would be cool that this test fails when we try to delete a non existant transaction
#[test]
fn delete_non_existant_transaction_by_id() {
    use rand::Rng;

    let mut conn: diesel::SqliteConnection = establish_connection();

    let t_id = rand::random();
    assert!(Transaction::remove_by_id(&t_id, &mut conn).is_some());
}

// TODO could be nice to have an option to list the transactions that have been deleted
#[test]
fn delete_transaction_by_name() {

    let mut conn: diesel::SqliteConnection = establish_connection();

    let name = "testname";
    let company = Some("TestCo");

    let tr: Transaction = Transaction::create(  
        name, company, None, None, 20.0, None, None, None, &mut conn,
    )
    .expect("Failed to create trans");

    assert!(Transaction::remove_by_name(tr.name.as_str(), &mut conn).is_some());

    assert_eq!(Transaction::list(&mut conn).len(), 0);
}
