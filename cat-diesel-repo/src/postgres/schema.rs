// @generated automatically by Diesel CLI.

diesel::table! {
    chats (chat_id) {
        chat_id -> Int8,
        #[max_length = 250]
        name -> Nullable<Varchar>,
        enable_push -> Bool,
        #[max_length = 250]
        title -> Nullable<Varchar>,
    }
}
