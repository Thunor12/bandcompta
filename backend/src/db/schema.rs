// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Integer,
        name -> Text,
        company -> Nullable<Text>,
        executed -> Integer,
        exec_date -> Nullable<Text>,
        price_full_tax -> Float,
        tax_amount -> Nullable<Float>,
        tag -> Nullable<Text>,
        invoice_path -> Nullable<Text>,
    }
}
