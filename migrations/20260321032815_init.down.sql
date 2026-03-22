BEGIN;

DROP TABLE IF EXISTS messenger.messages;

DROP TABLE IF EXISTS messenger.chat_participants;

DROP TABLE IF EXISTS messenger.chats;

DROP TABLE IF EXISTS messenger.users;

DROP SCHEMA IF EXISTS messenger;

COMMIT;
