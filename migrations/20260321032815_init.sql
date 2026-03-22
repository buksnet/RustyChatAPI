BEGIN;

CREATE SCHEMA IF NOT EXISTS messenger;

CREATE TABLE messenger.users
(
    id          SERIAL PRIMARY KEY,
    tag         VARCHAR(100) NOT NULL UNIQUE,
    name        VARCHAR(50)  NOT NULL,
    pwd_hash    VARCHAR(60)  NOT NULL,
    profile_pic TEXT         NOT NULL DEFAULT 'default.png',
    status      VARCHAR(200),
    is_online   BOOLEAN      NOT NULL DEFAULT false,
    join_date   DATE         NOT NULL DEFAULT NOW(),
    last_online TIMESTAMP    NOT NULL DEFAULT NOW(),
    birthday    DATE,
    email       VARCHAR(320) NOT NULL UNIQUE
);

CREATE TABLE messenger.chats
(
    id         SERIAL PRIMARY KEY,
    title      VARCHAR(200) NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE TABLE messenger.chat_participants
(
    chat_id   INTEGER REFERENCES messenger.chats (id) ON DELETE CASCADE,
    user_id   INTEGER REFERENCES messenger.users (id) ON DELETE CASCADE,
    joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, user_id)
);

CREATE TABLE messenger.messages
(
    id         SERIAL PRIMARY KEY,
    chat_id    INTEGER   NOT NULL REFERENCES messenger.chats (id) ON DELETE CASCADE,
    sender_id  INTEGER   NOT NULL REFERENCES messenger.users (id) ON DELETE CASCADE,
    content    TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

COMMIT;