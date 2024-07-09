-- This file should undo anything in `up.sql`
-- Remove the two words from the word table
DELETE FROM word WHERE word IN ('stylo', 'table');