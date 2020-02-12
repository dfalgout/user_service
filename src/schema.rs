table! {
    users (id) {
        id -> Int4,
        user_id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
