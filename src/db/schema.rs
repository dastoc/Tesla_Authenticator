diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}