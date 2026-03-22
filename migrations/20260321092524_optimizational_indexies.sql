BEGIN;

-- Для быстрого поиска чатов пользователя
CREATE INDEX idx_participants_user ON messenger.chat_participants (user_id);

-- Для быстрой сортировки сообщений по чату
CREATE INDEX idx_messages_chat_time ON messenger.messages (chat_id, created_at DESC);

COMMIT;

