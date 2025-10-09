UPDATE chats 
SET 
    name = $1, 
    enable_push = $2, 
    title = $3 
WHERE chat_id = $4;