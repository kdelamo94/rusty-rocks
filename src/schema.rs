table! {
    rusty_users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        rusty_password -> Varchar,
        rusty_role -> Varchar,
    }
}

table! {
    temporary_table (id) {
        id -> Int4,
        size -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    rusty_users,
    temporary_table,
);
