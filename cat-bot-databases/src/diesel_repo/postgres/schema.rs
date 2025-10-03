diesel::table! {
    chats (chat_id) {
        chat_id -> Int8,
        name -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        enable_push -> Bool,
    }
}
