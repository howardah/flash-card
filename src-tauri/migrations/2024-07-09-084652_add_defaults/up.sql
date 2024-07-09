-- Your SQL goes here
-- Change the 'word' table to have default times for 'created_at' and 'updated_at'
ALTER TABLE word
    RENAME TO word_old;

CREATE TABLE word (
    id INTEGER NOT NULL PRIMARY KEY,
    word VARCHAR NOT NULL,
    gender VARCHAR(191) NOT NULL,
    translation VARCHAR(191),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO word (id, word, gender, translation, created_at, updated_at)
    SELECT id, word, gender, translation, created_at, updated_at FROM word_old;

DROP TABLE word_old;

-- Change the 'set' table to have default times for 'created_at' and 'updated_at'
ALTER TABLE `set`
    RENAME TO set_old;

CREATE TABLE `set` (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR(191) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO `set` (id, name, created_at, updated_at)
    SELECT id, name, created_at, updated_at FROM set_old;

DROP TABLE set_old;

-- Create triggers to update 'updated_at' column on update
CREATE TRIGGER update_word_updated_at
    AFTER UPDATE ON word
    FOR EACH ROW
    BEGIN
        UPDATE word SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
    END;

CREATE TRIGGER update_set_updated_at
    AFTER UPDATE ON `set`
    FOR EACH ROW
    BEGIN
        UPDATE `set` SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
    END;
