-- This file should undo anything in `up.sql`
-- Revert changes to the 'word' table
ALTER TABLE word
    RENAME TO word_new;

CREATE TABLE word (
    id INTEGER NOT NULL PRIMARY KEY,
    word VARCHAR NOT NULL,
    gender VARCHAR(191) NOT NULL,
    translation VARCHAR(191),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO word (id, word, gender, translation, created_at, updated_at)
    SELECT id, word, gender, translation, created_at, updated_at FROM word_new;

DROP TABLE word_new;

-- Revert changes to the 'set' table
ALTER TABLE `set`
    RENAME TO set_new;

CREATE TABLE `set` (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR(191) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO `set` (id, name, created_at, updated_at)
    SELECT id, name, created_at, updated_at FROM set_new;

DROP TABLE set_new;

-- Drop triggers
DROP TRIGGER IF EXISTS update_word_updated_at;
DROP TRIGGER IF EXISTS update_set_updated_at;
