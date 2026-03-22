BEGIN;

CREATE TABLE IF NOT EXISTS messenger.chat_moderators
(
    chat_id          INTEGER NOT NULL REFERENCES messenger.chats (id) ON DELETE CASCADE,
    moderator_id     INTEGER NOT NULL REFERENCES messenger.users (id) ON DELETE CASCADE,
    permission_level INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (chat_id, moderator_id)
);

-- Добавление поля в старые таблицы для поддержки включения и выключения аккаунтов, подтверждения аккаунта
CREATE TYPE USER_PERMISSION AS ENUM ('banned', 'inactive', 'pending', 'active', 'moderator');

ALTER TABLE messenger.users
    ADD COLUMN permissions USER_PERMISSION NOT NULL DEFAULT 'pending';

COMMIT;
