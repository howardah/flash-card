-- Remove duplicates
DELETE FROM word
WHERE id NOT IN (
    SELECT min(id)
    FROM word
    GROUP BY word, gender
);

-- Add unique constraint
CREATE UNIQUE INDEX unique_word_gender ON word (word, gender);